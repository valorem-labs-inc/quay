use crate::configuration::Settings;
use crate::seaport::Seaport;
use crate::startup::get_connection_pool;
use crate::structs::Network;
use anyhow::Result;
use ethers::abi::AbiEncode;
use ethers::prelude::*;
use ethers::providers::Provider;
use sqlx::PgPool;
use std::str::FromStr;
use std::sync::Arc;
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
    order_hash: &String,
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
    order_hash: &String,
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
    provider: Arc<Provider<Http>>,
    seaport: Seaport<Provider<Http>>,
    pool: Arc<PgPool>,
}

impl Indexer {
    pub async fn new(configuration: Settings) -> Result<Self, std::io::Error> {
        let pool = Arc::new(get_connection_pool(&configuration.database));

        let provider: Arc<Provider<Http>> = Arc::new(Provider::new(
            Http::from_str(configuration.rpc.uri.as_str()).unwrap(),
        ));

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

    async fn process_block(&self, block_number: U64) -> Result<()> {
        // TODO(Enqueue and await the futures of all db operations in a vec for a speedup)
        if block_number >= U64::from(10835536) {
            let fulfilled = self
                .seaport
                .order_fulfilled_filter()
                .from_block(block_number)
                .to_block(block_number)
                .query()
                .await?;
            let cancelled = self
                .seaport
                .order_cancelled_filter()
                .from_block(block_number)
                .to_block(block_number)
                .query()
                .await?;
            for cancellation in cancelled {
                let order_hash = cancellation.order_hash.encode_hex();
                debug!("Cancellation for {}", &order_hash);
                update_order_cancellation(&self.pool, &order_hash, true).await?;
            }
            for fulfillment in fulfilled {
                let order_hash = fulfillment.order_hash.encode_hex();
                debug!("Fulfillment for {}", &order_hash);
                update_order_fulfillment(&self.pool, &order_hash, true).await?;
            }
        }
        Ok(())
    }

    async fn run(&mut self) -> Result<()> {
        let batch = 50;
        let network_id = 4;
        let watcher = self.provider.clone();
        let mut block_stream = watcher.watch_blocks().await?;
        // One block before the eth registrar controller was deployed
        // was block # 9380470
        let start_indexing_block: i64 = 10835536;
        init_network(&self.pool, &network_id, &start_indexing_block).await?;
        let mut next_block_to_process =
            U64::from(get_network(&self.pool, &network_id).await?.indexed_block);
        let mut block_number: U64;
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
                    next_block_to_process += U64::from(network_id);
                }
                futures::future::join_all(tasks).await;
                update_network(&self.pool, &network_id, &(end_batch.as_u64() as i64 + 1)).await?;
            }
        }
        Ok(())
    }
}

// This is wrapped up in a thread pool for call by the binary.
#[tokio::main]
pub async fn run(configuration: Settings) -> Result<()> {
    let mut indexer = Indexer::new(configuration).await?;

    // Run the roller
    indexer.run().await?;

    // Exit cleanly
    Ok(())
}
