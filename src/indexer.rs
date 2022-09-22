use crate::configuration::Settings;
use crate::seaport::{
    CounterIncrementedFilter, OrderCancelledFilter, OrderFulfilledFilter, Seaport,
};
use crate::startup::get_connection_pool;
use crate::structs::Network;
use anyhow::Result;
use ethers::abi::AbiEncode;
use ethers::prelude::*;
use ethers::providers::Provider;
use sqlx::PgPool;

use futures::future::try_join_all;
use futures::try_join;
use log::warn;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::sleep;

use std::time::Duration;

use tracing::{debug, info};

// TODO(Network id and indexed block should be U64 types, but we need sqlx bindings for those first)

pub async fn init_network(
    pool: &PgPool,
    network_id: &i32,
    indexed_block: &i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"INSERT INTO networks (network, indexed_block) VALUES ($1, $2) ON CONFLICT DO NOTHING"#,
        network_id,
        indexed_block
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

pub async fn get_network(pool: &PgPool, network_id: &i32) -> Result<Network, sqlx::Error> {
    let network: Network = sqlx::query_as!(
        Network,
        r#"SELECT network, indexed_block FROM networks WHERE network = $1"#,
        network_id
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(network)
}

pub async fn update_network(
    pool: &PgPool,
    network_id: &i32,
    indexed_block: &i64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE networks SET indexed_block = $2 WHERE network = $1"#,
        network_id,
        indexed_block
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

pub async fn update_order_fulfillment(
    pool: &PgPool,
    order_hash: String,
    fulfilled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE orders SET finalized = $2 WHERE hash = $1"#,
        order_hash,
        fulfilled
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

pub async fn increment_offerer_counter(
    pool: &PgPool,
    offerer: Address,
    counter: U256,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE orders SET cancelled = true WHERE offerer = $1 AND counter < $2"#,
        offerer.encode_hex(),
        counter.as_u64() as i64
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

pub async fn update_order_cancellation(
    pool: &PgPool,
    order_hash: String,
    cancelled: bool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE orders SET cancelled = $2 WHERE hash = $1"#,
        order_hash,
        cancelled
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
    })?;
    Ok(())
}

struct Indexer {
    provider: Arc<Provider<RetryClient<Http>>>,
    seaport: Seaport<Provider<RetryClient<Http>>>,
    seaport_deploy_block: i64,
    chain_id: i32,
    pool: Arc<PgPool>,
}

impl Indexer {
    pub async fn new(configuration: Settings) -> Result<Self, std::io::Error> {
        let pool = Arc::new(get_connection_pool(&configuration.database));

        let provider: Arc<Provider<RetryClient<Http>>> =
            Arc::new(Provider::new_client(configuration.rpc.uri.as_str(), 3, 10).unwrap());

        let seaport = Seaport::new(
            H160::from_str("0x00000000006c3852cbEf3e08E8dF289169EdE581").unwrap(),
            provider.clone(),
        );

        let seaport_deploy_block = configuration.indexer.seaport_deploy_block;

        let chain_id = configuration.rpc.chain_id;

        Ok(Self {
            provider,
            seaport,
            seaport_deploy_block,
            chain_id,
            pool,
        })
    }

    async fn get_block_events(
        &self,
        block_number: U64,
    ) -> Result<
        (
            Vec<OrderFulfilledFilter>,
            Vec<OrderCancelledFilter>,
            Vec<CounterIncrementedFilter>,
        ),
        ContractError<Provider<RetryClient<Http>>>,
    > {
        let block_number = block_number;
        let fulfilled = self
            .seaport
            .order_fulfilled_filter()
            .from_block(block_number)
            .to_block(block_number);
        let cancelled = self
            .seaport
            .order_cancelled_filter()
            .from_block(block_number)
            .to_block(block_number);
        let counter_updated = self
            .seaport
            .counter_incremented_filter()
            .from_block(block_number)
            .to_block(block_number);
        let results = try_join!(
            fulfilled.query(),
            cancelled.query(),
            counter_updated.query()
        )
        .map_err(|e| {
            tracing::error!("Failed to get events: {:?}", e);
            e
        })?;
        Ok(results)
    }

    #[allow(clippy::type_complexity)]
    async fn process_block(&self, block_number: U64) -> Result<(), anyhow::Error> {
        // TODO(Build one sql transaction per block?)
        let (fulfilled, cancelled, counter_updated) = self.get_block_events(block_number).await?;
        let mut cancellations = vec![];
        for cancellation in cancelled {
            let order_hash = cancellation.order_hash.encode_hex();
            debug!("Cancellation for {}", &order_hash);
            cancellations.push(update_order_cancellation(&self.pool, order_hash, true));
        }
        let mut fulfillments = vec![];
        for fulfillment in fulfilled {
            let order_hash = fulfillment.order_hash.encode_hex();
            debug!("Fulfillment for {}", &order_hash);
            fulfillments.push(update_order_fulfillment(
                &self.pool,
                fulfillment.order_hash.encode_hex(),
                true,
            ));
        }
        let mut counter_updates = vec![];
        for counter_update in counter_updated {
            counter_updates.push(increment_offerer_counter(
                &self.pool,
                counter_update.offerer,
                counter_update.new_counter,
            ));
        }
        let result = try_join!(
            try_join_all(cancellations),
            try_join_all(fulfillments),
            try_join_all(counter_updates)
        );

        result.unwrap();
        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        // TODO(Convert from batch to ordered queue with concurrency)
        let batch = 16;
        let watcher = self.provider.clone();
        let mut block_stream = watcher.watch_blocks().await?;
        // One block before the eth registrar controller was deployed
        // was block # 9380470
        let deploy_block = self.seaport_deploy_block;
        init_network(&self.pool, &self.chain_id, &deploy_block).await?;
        let mut next_block_to_process =
            U64::from(get_network(&self.pool, &self.chain_id).await?.indexed_block);
        let mut block_number: U64;
        info!("Waiting for next block from eth node");
        while block_stream.next().await.is_some() {
            block_number = self
                .provider
                .get_block(BlockNumber::Latest)
                .await
                .unwrap()
                .unwrap()
                .number
                .unwrap();
            info!("Got block {}", block_number);
            while next_block_to_process <= block_number {
                let blocks_remaining = block_number - next_block_to_process;
                let end_batch = if blocks_remaining < U64::from(batch) {
                    next_block_to_process + blocks_remaining
                } else {
                    next_block_to_process + U64::from(batch)
                };
                info!(
                    "Processing block: {} to {} of {}",
                    next_block_to_process, end_batch, block_number
                );
                let mut tasks = vec![];
                while next_block_to_process <= end_batch {
                    tasks.push(self.process_block(next_block_to_process));
                    next_block_to_process += U64::from(1);
                }
                try_join_all(tasks).await?;
                update_network(&self.pool, &self.chain_id, &(end_batch.as_u64() as i64 + 1))
                    .await?;
            }
        }
        Ok(())
    }
}

// This is wrapped up in a thread pool for call by the binary.
#[tokio::main]
pub async fn run(configuration: Settings) -> Result<(), anyhow::Error> {
    // TODO(Handle SIGINT, SIGKILL gracefully)
    // We want to keep the indexer running if DB or RPC times out
    loop {
        let mut indexer = Indexer::new(configuration.clone()).await?;
        // Let's index and throw away errors in case of a db timeout or whatever
        info!("Running indexer");
        let _result = indexer.run().await;
        warn!("Indexer stopped/timed out, restarting!");
        // Sleep 1 second in case of a crash
        sleep(Duration::from_secs(1)).await;
    }
}
