use axum::extract::FromRef;
use bb8::Pool;
use ethers::providers::{Http, Provider};
use sqlx::PgPool;

use crate::bindings::Seaport;
use crate::redis_pool::RedisConnectionManager;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_pool: Pool<RedisConnectionManager>,
    pub rpc: Provider<Http>,
    pub seaport: Seaport<Provider<Http>>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> PgPool {
        app_state.db_pool.clone()
    }
}
impl FromRef<AppState> for Pool<RedisConnectionManager> {
    fn from_ref(app_state: &AppState) -> Pool<RedisConnectionManager> {
        app_state.redis_pool.clone()
    }
}
impl FromRef<AppState> for Provider<Http> {
    fn from_ref(app_state: &AppState) -> Provider<Http> {
        app_state.rpc.clone()
    }
}
impl FromRef<AppState> for Seaport<Provider<Http>> {
    fn from_ref(app_state: &AppState) -> Seaport<Provider<Http>> {
        app_state.seaport.clone()
    }
}
