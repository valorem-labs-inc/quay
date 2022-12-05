use std::fmt::{Debug, Formatter};

use axum_sessions::async_session::{async_trait, serde_json, Result, Session, SessionStore};
use redis::aio::ConnectionManager;
use redis::{Client, Cmd, FromRedisValue, RedisResult};

/// This redis session store uses a multiplexed connection to redis with an auto-reconnect feature.
/// # RedisSessionStore
/// This redis session store uses a multiplexed connection to redis with an auto-reconnect feature.
#[derive(Clone)]
pub struct RedisSessionStore {
    /// A `ConnectionManager` that wraps a multiplexed connection and automatically reconnects to the server when necessary.
    connection: ConnectionManager,
    /// The prefix to be used for all session keys in Redis.
    prefix: Option<String>,
}

impl RedisSessionStore {
    /// Creates a new `RedisSessionStore` from a redis uri string.
    pub async fn from_uri(uri: &str, prefix: Option<String>) -> RedisResult<Self> {
        let connection = ConnectionManager::new(Client::open(uri).unwrap()).await?;
        Ok(Self { connection, prefix })
    }

    /// Creates a new `RedisSessionStore` from a `redis::Client`.
    pub async fn from_client(client: redis::Client, prefix: Option<String>) -> RedisResult<Self> {
        Ok(Self {
            connection: client.get_tokio_connection_manager().await?,
            prefix,
        })
    }

    /// Constructs a new `RedisSessionStore` instance.
    ///
    /// # Arguments
    ///
    /// * `connection` - The `ConnectionManager` to be used for Redis connections.
    /// * `prefix` - An optional prefix to be used for all session keys in Redis.
    pub fn new(connection: ConnectionManager, prefix: Option<String>) -> Self {
        Self { connection, prefix }
    }

    /// Sets the prefix for session keys in Redis.
    ///
    /// # Arguments
    ///
    /// * `prefix` - The prefix to be used for all session keys in Redis.
    pub fn with_prefix(mut self, prefix: impl AsRef<str>) -> Self {
        self.prefix = Some(prefix.as_ref().to_owned());
        self
    }

    /// Returns the session keys in Redis that match the prefix.
    async fn ids(&self) -> Result<Vec<String>> {
        Ok(self
            .execute_command(&mut Cmd::keys(self.prefix_key("*")))
            .await?)
    }

    /// Returns the number of sessions in this store.
    pub async fn count(&self) -> Result<usize> {
        if self.prefix.is_none() {
            Ok(self.execute_command(&mut redis::cmd("DBSIZE")).await?)
        } else {
            Ok(self.ids().await?.len())
        }
    }

    /// Returns the time-to-live (TTL) for the given session.
    #[cfg(test)]
    async fn ttl_for_session(&self, session: &Session) -> Result<usize> {
        Ok(self
            .execute_command(&mut Cmd::ttl(self.prefix_key(&session.id())))
            .await?)
    }

    /// Prefixes the given key with the configured session key prefix.
    fn prefix_key(&self, key: impl AsRef<str>) -> String {
        if let Some(ref prefix) = self.prefix {
            format!("{}{}", prefix, key.as_ref())
        } else {
            key.as_ref().into()
        }
    }

    /// Execute Redis command and retry once in certain cases.
    ///
    /// `ConnectionManager` automatically reconnects when it encounters an error talking to Redis.
    /// The request that bumped into the error, though, fails.
    ///
    /// This is generally OK, but there is an unpleasant edge case: Redis client timeouts. The
    /// server is configured to drop connections who have been active longer than a pre-determined
    /// threshold. `redis-rs` does not proactively detect that the connection has been dropped - you
    /// only find out when you try to use it.
    ///
    /// This helper method catches this case (`.is_connection_dropped`) to execute a retry. The
    /// retry will be executed on a fresh connection, therefore it is likely to succeed (or fail for
    /// a different more meaningful reason).
    async fn execute_command<T: FromRedisValue>(&self, cmd: &mut Cmd) -> RedisResult<T> {
        let mut can_retry = true;

        loop {
            match cmd.query_async(&mut self.connection.clone()).await {
                Ok(value) => return Ok(value),
                Err(err) => {
                    if can_retry && err.is_connection_dropped() {
                        tracing::debug!(
                            "Connection dropped while trying to talk to Redis. Retrying."
                        );

                        // Retry at most once
                        can_retry = false;

                        continue;
                    } else {
                        return Err(err);
                    }
                }
            }
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
    /// Loads the session with the given cookie value.
    ///
    /// # Arguments
    ///
    /// * `cookie_value` - The cookie value to load the session for.
    ///
    /// # Returns
    ///
    /// If the session exists, returns a `Some` with the session. Otherwise, returns `None`.
    async fn load_session(&self, cookie_value: String) -> Result<Option<Session>> {
        // Extract the session id from the cookie value.
        let id = Session::id_from_cookie_value(&cookie_value)?;

        // Attempt to get the session data from Redis.
        let record: Option<String> = self
            .execute_command(&mut Cmd::get(self.prefix_key(id)))
            .await?;

        // If a session was found, deserialize it and return it. Otherwise, return `None`.
        match record {
            Some(value) => Ok(serde_json::from_str(&value)?),
            None => Ok(None),
        }
    }

    /// Stores the given session.
    ///
    /// # Arguments
    ///
    /// * `session` - The session to store.
    ///
    /// # Returns
    ///
    /// If the session was successfully stored, returns a `Some` with the session's cookie value. Otherwise, returns `None`.
    async fn store_session(&self, session: Session) -> Result<Option<String>> {
        // Get the session id with the prefix applied.
        let id = self.prefix_key(session.id());
        // Serialize the session.
        let string = serde_json::to_string(&session)?;

        // Set the session in Redis with the appropriate expiry time.
        match session.expires_in() {
            None => self.execute_command(&mut Cmd::set(id, string)).await?,
            Some(expiry) => {
                self.execute_command(&mut Cmd::set_ex(id, string, expiry.as_secs() as usize))
                    .await?
            }
        };

        // Return the session's cookie value.
        Ok(session.into_cookie_value())
    }

    /// Destroys the given session.
    ///
    /// # Arguments
    ///
    /// * `session` - The session to destroy.
    ///
    /// # Returns
    ///
    /// If the session was successfully destroyed, returns `Ok(())`. Otherwise, returns an error.
    async fn destroy_session(&self, session: Session) -> Result {
        // Get the session id with the prefix applied.
        let key = self.prefix_key(session.id());
        // Delete the session from Redis.
        self.execute_command(&mut Cmd::del(key)).await?;
        Ok(())
    }

    /// Clears all sessions in the store.
    ///
    /// If `prefix` is not set, this will clear the entire Redis database.
    /// Otherwise, it will only clear the sessions with the specified prefix.
    async fn clear_store(&self) -> Result {
        if self.prefix.is_none() {
            let _: () = self.execute_command(&mut redis::cmd("FLUSHDB")).await?;
        } else {
            let ids = self.ids().await?;
            if !ids.is_empty() {
                self.execute_command(&mut Cmd::del(ids)).await?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use redis::Client;
    use std::time::Duration;

    use super::*;

    async fn test_store() -> RedisSessionStore {
        let store =
            RedisSessionStore::from_uri("redis://127.0.0.1", Some(ulid::Ulid::new().to_string()))
                .await
                .unwrap();
        store.clear_store().await.unwrap();
        store
    }

    #[tokio::test]
    async fn creating_a_new_session_with_no_expiry() -> Result {
        let store = test_store().await;
        let mut session = Session::new();
        session.insert("key", "value")?;
        let cloned = session.clone();
        let cookie_value = store.store_session(session).await?.unwrap();

        let loaded_session = store.load_session(cookie_value).await?.unwrap();
        assert_eq!(cloned.id(), loaded_session.id());
        assert_eq!("value", &loaded_session.get::<String>("key").unwrap());

        assert!(!loaded_session.is_expired());
        Ok(())
    }

    #[tokio::test]
    async fn updating_a_session() -> Result {
        let store = test_store().await;
        let mut session = Session::new();

        session.insert("key", "value")?;
        let cookie_value = store.store_session(session).await?.unwrap();

        let mut session = store.load_session(cookie_value.clone()).await?.unwrap();
        session.insert("key", "other value")?;
        assert_eq!(None, store.store_session(session).await?);

        let session = store.load_session(cookie_value.clone()).await?.unwrap();
        assert_eq!(&session.get::<String>("key").unwrap(), "other value");

        assert_eq!(1, store.count().await.unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn updating_a_session_extending_expiry() -> Result {
        let store = test_store().await;
        let mut session = Session::new();
        session.expire_in(Duration::from_secs(5));
        let original_expires = session.expiry().unwrap().clone();
        let cookie_value = store.store_session(session).await?.unwrap();

        let mut session = store.load_session(cookie_value.clone()).await?.unwrap();
        let ttl = store.ttl_for_session(&session).await?;
        assert!(ttl > 3 && ttl < 5);

        assert_eq!(session.expiry().unwrap(), &original_expires);
        session.expire_in(Duration::from_secs(10));
        let new_expires = session.expiry().unwrap().clone();
        store.store_session(session).await?;

        let session = store.load_session(cookie_value.clone()).await?.unwrap();
        let ttl = store.ttl_for_session(&session).await?;
        assert!(ttl > 8 && ttl < 10);
        assert_eq!(session.expiry().unwrap(), &new_expires);

        assert_eq!(1, store.count().await.unwrap());

        tokio::time::sleep(Duration::from_secs(10)).await;
        assert_eq!(0, store.count().await.unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn creating_a_new_session_with_expiry() -> Result {
        let store = test_store().await;
        let mut session = Session::new();
        session.expire_in(Duration::from_secs(3));
        session.insert("key", "value")?;
        let cloned = session.clone();

        let cookie_value = store.store_session(session).await?.unwrap();

        assert!(store.ttl_for_session(&cloned).await? > 1);

        let loaded_session = store.load_session(cookie_value.clone()).await?.unwrap();
        assert_eq!(cloned.id(), loaded_session.id());
        assert_eq!("value", &loaded_session.get::<String>("key").unwrap());

        assert!(!loaded_session.is_expired());

        tokio::time::sleep(Duration::from_secs(2)).await;
        assert_eq!(None, store.load_session(cookie_value).await?);

        Ok(())
    }

    #[tokio::test]
    async fn destroying_a_single_session() -> Result {
        let store = test_store().await;
        for _ in 0..3i8 {
            store.store_session(Session::new()).await?;
        }

        let cookie = store.store_session(Session::new()).await?.unwrap();
        assert_eq!(4, store.count().await?);
        let session = store.load_session(cookie.clone()).await?.unwrap();
        store.destroy_session(session.clone()).await.unwrap();
        assert_eq!(None, store.load_session(cookie).await?);
        assert_eq!(3, store.count().await?);

        // attempting to destroy the session again is not an error
        assert!(store.destroy_session(session).await.is_ok());
        Ok(())
    }

    #[tokio::test]
    async fn clearing_the_whole_store() -> Result {
        let store = test_store().await;
        for _ in 0..3i8 {
            store.store_session(Session::new()).await?;
        }

        //assert_eq!(3, store.count().await?);
        store.clear_store().await.unwrap();
        assert_eq!(0, store.count().await?);

        Ok(())
    }

    #[tokio::test]
    async fn prefixes() -> Result {
        test_store().await; // clear the db

        let client = Client::open("redis://127.0.0.1").unwrap();
        let cm = ConnectionManager::new(client).await.unwrap();
        let store = RedisSessionStore::new(cm.clone(), Some("sessions/".to_string()));

        store.clear_store().await?;

        for _ in 0..3i8 {
            store.store_session(Session::new()).await?;
        }

        let mut session = Session::new();

        session.insert("key", "value")?;
        let cookie_value = store.store_session(session).await?.unwrap();

        let mut session = store.load_session(cookie_value.clone()).await?.unwrap();
        session.insert("key", "other value")?;
        assert_eq!(None, store.store_session(session).await?);

        let session = store.load_session(cookie_value.clone()).await?.unwrap();
        assert_eq!(&session.get::<String>("key").unwrap(), "other value");

        assert_eq!(4, store.count().await.unwrap());

        let other_store = RedisSessionStore::new(cm.clone(), Some("other_namespace/".to_string()));

        assert_eq!(0, other_store.count().await.unwrap());
        for _ in 0..3i8 {
            other_store.store_session(Session::new()).await?;
        }

        other_store.clear_store().await?;

        assert_eq!(0, other_store.count().await?);
        assert_eq!(4, store.count().await?);

        Ok(())
    }
}
