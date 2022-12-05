// From https://github.com/djc/bb8/blob/main/redis/src/lib.rs
// Brought in here to use tokio-native-tls-comp and later tokio-rustls-comp
// TODO(https://github.com/redis-rs/redis-rs/pull/725)
// Rather than tokio-comp

use async_trait::async_trait;
use bb8;
use redis::{aio::Connection, ErrorKind};
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
