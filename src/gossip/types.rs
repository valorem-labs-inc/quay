use crate::structs::{DBConsideration, DBOffer};
use ssz_rs::prelude::*;
use thiserror::Error;

type H160 = Vector<u8, 20>;
type H256 = Vector<u8, 32>;
type Root = H256;
type Signature = Vector<u8, 65>;

/// An item contained in an offer.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OfferItem {
    /// The item type.
    pub item_type: u8,
    /// The token.
    pub token: H160,
    /// The identifier or criteria for the item.
    pub identifier_or_criteria: U256,
    /// The start amount of the item.
    pub start_amount: U256,
    /// The end amount of the item.
    pub end_amount: U256,
}

/// A consideration for an item.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct ConsiderationItem {
    /// The offered item.
    #[serde(flatten)]
    pub offer: OfferItem,
    /// The recipient for the item.
    pub recipient: H160,
}

/// A seaport order in a Seaport Gossip Network compatible format, encoded using SSZ.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub offer: List<OfferItem, 100>,
    pub consideration: List<ConsiderationItem, 100>,
    pub offerer: H160,
    pub signature: Signature,
    pub order_type: u8,
    pub start_time: u32,
    pub end_time: u32,
    pub counter: U256,
    pub salt: U256,
    pub conduit_key: H256,
    pub zone: H160,
    pub zone_hash: H256,
    pub chain_id: U256,
    // Basic order types
    pub additional_recipients: List<H160, 50>,
    // Advanced order types
    pub numerator: U256,
    pub denominator: U256,
    pub extra_data: H256,
}

/// Gossipsub event indicating that the local node is sending offers.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Orders {
    #[serde(flatten)]
    req_id: u64,
    orders: List<Order, 100>,
}

/// Standard Seaport Gossip Network event format.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GossipsubEvent {
    #[serde(flatten)]
    event: u8,
    order_hash: Root,
    order: Order,
    block_number: u64,
    block_hash: Root,
}

/// A filter for an specific order, using its hash.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OrderFilter {
    #[serde(flatten)]
    key: u8,
    value: H160,
}

/// Options for more granular filtering when querying orders.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GetOrdersOpts {
    #[serde(flatten)]
    side: u8,
    count: u32,
    offset: u32,
    sort: u8,
    filter: List<OrderFilter, 20>,
}

/// Gossipsub message that queries for a single or multiple orders.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OrderQuery {
    #[serde(flatten)]
    req_id: u64,
    address: H160,
    opts: GetOrdersOpts,
}

/// Gossipsub message that communicates the amount of orders the local node has.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OrderCount {
    #[serde(flatten)]
    req_id: u64,
    count: u64,
}

/// Gossipsub message that communicates the amount of order hashes for an specific address.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OrderHashes {
    #[serde(flatten)]
    req_id: u64,
    hashes: List<H256, 1_000_000>,
}

/// Gossipsub message that queries for the criteria on an order.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GetCriteria {
    #[serde(flatten)]
    req_id: u64,
    hash: H256,
}

/// Gossipsub message that communicates the criteria of an order.
/// Response to GetCriteria.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CriteriaItem {
    #[serde(flatten)]
    req_id: u64,
    hash: H256,
    items: List<U256, 10_000_000>,
}

/// Possible errors when converting from a DB Offer to a ssz offer item.
#[derive(Error, Debug)]
pub enum OfferConversionError {
    /// Hex decoding failed.
    #[error("Hex decoding error")]
    HexDecodeError(#[from] hex::FromHexError),
    /// Ssz serializing failed.
    #[error("Ssz serialization error")]
    SszSerializationError(#[from] ssz_rs::DeserializeError),
}

impl TryFrom<DBOffer> for OfferItem {
    type Error = OfferConversionError;

    fn try_from(value: DBOffer) -> Result<Self, Self::Error> {
        let token: H160 = hex::decode(value.token)?.try_into().unwrap(); // result is infallible here

        Ok(OfferItem {
            item_type: value.item_type as u8,
            token,
            identifier_or_criteria: U256::try_from_bytes_le(&hex::decode(
                value.identifier_or_criteria,
            )?)?,
            start_amount: U256::try_from_bytes_le(&hex::decode(value.start_amount)?)?,
            end_amount: U256::try_from_bytes_le(&hex::decode(value.end_amount)?)?,
        })
    }
}

impl TryFrom<DBConsideration> for ConsiderationItem {
    type Error = OfferConversionError;

    fn try_from(value: DBConsideration) -> Result<Self, Self::Error> {
        let offer: OfferItem = DBOffer {
            position: value.position,
            item_type: value.item_type,
            token: value.token,
            identifier_or_criteria: value.identifier_or_criteria,
            start_amount: value.start_amount,
            end_amount: value.end_amount,
        }
        .try_into()?;

        let recipient: H160 = hex::decode(value.recipient)?.try_into().unwrap(); // result infallible here

        Ok(ConsiderationItem { offer, recipient })
    }
}
