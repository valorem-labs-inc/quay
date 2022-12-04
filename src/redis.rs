// From https://github.com/djc/bb8/blob/main/redis/src/lib.rs
// Brought in here to use tokio-native-tls-comp and later tokio-rustls-comp
// TODO(https://github.com/redis-rs/redis-rs/pull/725)
// Rather than tokio-comp

pub use bb8;
pub use redis;

use async_trait::async_trait;
use redis::{
    aio::{Connection, MultiplexedConnection},
    ErrorKind,
};
use redis::{Client, IntoConnectionInfo, RedisError};

/// A `bb8::ManageConnection` for `redis::Client::get_async_connection`.
#[derive(Clone, Debug)]
pub struct RedisConnectionManager {
    client: Client,
}

impl RedisConnectionManager {
    /// Create a new `RedisConnectionManager`.
    /// See `redis::Client::open` for a description of the parameter types.
    pub fn new<T: IntoConnectionInfo>(info: T) -> Result<RedisConnectionManager, RedisError> {
        Ok(RedisConnectionManager {
            client: Client::open(info.into_connection_info()?)?,
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisConnectionManager {
    type Connection = Connection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_tokio_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

/// A `bb8::ManageConnection` for `redis::Client::get_async_connection`.
#[derive(Clone, Debug)]
pub struct RedisMultiplexedConnectionManager {
    client: Client,
}

impl RedisMultiplexedConnectionManager {
    /// Create a new `RedisConnectionManager`.
    /// See `redis::Client::open` for a description of the parameter types.
    pub fn new<T: IntoConnectionInfo>(
        info: T,
    ) -> Result<RedisMultiplexedConnectionManager, RedisError> {
        Ok(RedisMultiplexedConnectionManager {
            client: Client::open(info.into_connection_info()?)?,
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisMultiplexedConnectionManager {
    // TODO(Because this is multiplexed, we could probably just use a single connection)
    // cloned, using the redis multixplexed connection manager to reconnect on drops
    // let's see as v0.2.0 develops, if we really need a multiplexed connection pool
    // or just a single managed multiplexed connection for non blocking operations
    type Connection = MultiplexedConnection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_multiplexed_tokio_connection().await
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}
