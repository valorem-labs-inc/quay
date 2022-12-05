use std::fmt::{Debug, Formatter};

use axum_sessions::async_session::{async_trait, serde_json, Result, Session, SessionStore};
use redis::aio::ConnectionManager;
use redis::{aio::Connection, AsyncCommands, Client, IntoConnectionInfo, RedisResult};

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    async fn test_store() -> RedisSessionStore {
        let client = Client::open("redis://127.0.0.1").unwrap();
        let store = RedisSessionStore::new(ConnectionManager::new(client).await.unwrap(), None);
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

        assert_eq!(3, store.count().await?);
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
