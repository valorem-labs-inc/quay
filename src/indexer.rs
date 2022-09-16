use crate::configuration::Settings;
use crate::seaport::{OrderCancelledFilter, OrderFulfilledFilter, Seaport};
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
use std::thread::sleep;
use std::time::Duration;

use tracing::{debug, info};

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

        Ok(Self {
            provider,
            seaport,
            pool,
        })
    }

    // The old ens registry was created on block 3327417
    // at address 0x314159265dd8dbb310642f98f50c066173c1259b
    // That indexed all registrations until 9380410
    // Then the new base registrar was deployed at address
    // 0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85
    // And at block 9380471 the new eth registrar controller was deployed at
    // 0x283Af0B28c62C092C9727F1Ee09c02CA627EB7F5
    // A migration contract was also deployed at block 9406409
    // at address 0x6109dd117aa5486605fc85e040ab00163a75c662

    async fn get_block_events(
        &self,
        block_number: U64,
    ) -> Result<
        (Vec<OrderFulfilledFilter>, Vec<OrderCancelledFilter>),
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
        let results = try_join!(fulfilled.query(), cancelled.query(),).map_err(|e| {
            tracing::error!("Failed to get events: {:?}", e);
            e
        })?;
        Ok(results)
    }

    #[allow(clippy::type_complexity)]
    async fn process_block(&self, block_number: U64) -> Result<(), anyhow::Error> {
        // TODO(Build one sql transaction per block?)
        let (fulfilled, cancelled) = self.get_block_events(block_number).await?;
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
        let result = try_join!(try_join_all(cancellations), try_join_all(fulfillments));

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
        let deploy_block: i64 = 14946473;
        init_network(&self.pool, &1, &deploy_block).await?;
        let mut next_block_to_process = U64::from(get_network(&self.pool, &1).await?.indexed_block);
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
                update_network(&self.pool, &1, &(end_batch.as_u64() as i64 + 1)).await?;
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
        let _result = indexer.run().await?;
        warn!("Indexer stopped/timed out, restarting!");
        // Sleep 1 second in case of a crash
        sleep(Duration::from_secs(1));
    }
}
