use axum_sessions::async_session::{async_trait, serde_json, Result, Session, SessionStore};
use redis::aio::ConnectionManager;
use redis::{aio::Connection, AsyncCommands, Client, IntoConnectionInfo, RedisResult};
use std::fmt::{Debug, Formatter};

/// This redis session store uses a multiplexed connection to redis with an auto-reconnect feature.
/// # RedisSessionStore
#[derive(Clone)]
pub struct RedisSessionStore {
    connection: ConnectionManager,
    prefix: Option<String>,
}

impl RedisSessionStore {
    pub fn new(connection: ConnectionManager, prefix: Option<String>) -> Self {
        Self { connection, prefix }
    }

    pub fn with_prefix(mut self, prefix: impl AsRef<str>) -> Self {
        self.prefix = Some(prefix.as_ref().to_owned());
        self
    }

    async fn ids(&self) -> Result<Vec<String>> {
        Ok(self.connection.clone().keys(self.prefix_key("*")).await?)
    }

    /// returns the number of sessions in this store
    pub async fn count(&self) -> Result<usize> {
        if self.prefix.is_none() {
            Ok(redis::cmd("DBSIZE")
                .query_async(&mut self.connection.clone())
                .await?)
        } else {
            Ok(self.ids().await?.len())
        }
    }

    async fn ttl_for_session(&self, session: &Session) -> Result<usize> {
        Ok(self
            .connection
            .clone()
            .ttl(self.prefix_key(session.id()))
            .await?)
    }

    fn prefix_key(&self, key: impl AsRef<str>) -> String {
        if let Some(ref prefix) = self.prefix {
            format!("{}{}", prefix, key.as_ref())
        } else {
            key.as_ref().into()
        }
    }
}

impl Debug for RedisSessionStore {
    // TODO(PR debug impl for ConnectionManager then add back .field("connection", &self.connection)
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RedisSessionStore")
            .field("prefix", &self.prefix)
            .finish()
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        let id = Session::id_from_cookie_value(&cookie_value)?;
        let record: Option<String> = self.connection.clone().get(self.prefix_key(id)).await?;
        match record {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(None),
        }
    }

    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        let id = self.prefix_key(session.id());
        let string = serde_json::to_string(&session)?;

        match session.expires_in() {
            None => self.connection.clone().set(id, string).await?,

            Some(expiry) => {
                self.connection
                    .clone()
                    .set_ex(id, string, expiry.as_secs() as usize)
                    .await?
            }
        };

        Ok(session.into_cookie_value())
    }

    async fn destroy_session(&self, session: Session) -> Result {
        let key = self.prefix_key(session.id().to_string());
        self.connection.clone().del(key).await?;
        Ok(())
    }

    async fn clear_store(&self) -> Result {
        if self.prefix.is_none() {
            let _: () = redis::cmd("FLUSHDB")
                .query_async(&mut self.connection.clone())
                .await?;
        } else {
            let ids = self.ids().await?;
            if !ids.is_empty() {
                self.connection.clone().del(ids).await?;
            }
        }
        Ok(())
    }
}
