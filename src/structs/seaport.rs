use ethers::types::H160;
use serde::{Deserialize, Deserializer, Serialize};

use crate::seaport::Order;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OrderQuery {
    pub asset_contract_address: H160,
    #[serde(deserialize_with = "token_ids_deserialize")]
    pub token_ids: Vec<String>,
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RetrieveResponse {
    pub orders: Vec<ComplexOrder>,
}
impl RetrieveResponse {
    pub fn from_db_struct(db_orders: &[DBOrder]) -> Result<RetrieveResponse, anyhow::Error> {
        Ok(RetrieveResponse {
            orders: db_orders
                .iter()
                .map(ComplexOrder::from_db_struct)
                .collect::<Result<Vec<ComplexOrder>, anyhow::Error>>()?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComplexOrder {
    pub protocol_data: Order,
}
impl ComplexOrder {
    pub fn from_db_struct(db_order: &DBOrder) -> Result<ComplexOrder, anyhow::Error> {
        Ok(ComplexOrder {
            protocol_data: Order::from_db_struct(db_order)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DBOrder {
    pub hash: String,
    pub offerer: String,
    pub zone: String,
    pub zone_hash: String,
    pub start_time: i64,
    pub end_time: i64,
    pub order_type: i32,
    pub total_original_consideration_items: i32,
    pub salt: String,
    pub conduit_key: String,
    pub signature: String,

    pub offers: Vec<DBOffer>,
    pub considerations: Vec<DBConsideration>,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DBConsideration {
    pub position: i32,
    pub item_type: i32,
    pub token: String,
    pub identifier_or_criteria: String,
    pub start_amount: String,
    pub end_amount: String,
    pub recipient: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, sqlx::Type, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct DBOffer {
    pub position: i32,
    pub item_type: i32,
    pub token: String,
    pub identifier_or_criteria: String,
    pub start_amount: String,
    pub end_amount: String,
}

fn token_ids_deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_sequence = String::deserialize(deserializer)?;
    Ok(str_sequence
        .split(',')
        .map(|item| item.to_owned())
        .collect())
}
