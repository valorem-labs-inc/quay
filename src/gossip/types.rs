use crate::structs::DBOffer;
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
    offer: OfferItem,
    /// The recipient for the item.
    recipient: H160,
}

/// A seaport order in a Seaport Gossip Network compatible format, encoded using SSZ.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    offer: List<OfferItem, 100>,
    consideration: List<ConsiderationItem, 100>,
    offerer: H160,
    signature: Signature,
    order_type: u8,
    start_time: u32,
    end_time: u32,
    counter: U256,
    salt: U256,
    conduit_key: H256,
    zone: H160,
    zone_hash: H256,
    chain_id: U256,
    // Basic order types
    additional_recipients: List<H160, 50>,
    // Advanced order types
    numerator: U256,
    denominator: U256,
    extra_data: H256,
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
        Ok(OfferItem {
            item_type: value.item_type as u8,
            token: hex::decode(value.token)?
                .try_into()
                .expect("Invalid length"),
            identifier_or_criteria: U256::try_from_bytes_le(&hex::decode(
                value.identifier_or_criteria,
            )?)?,
            start_amount: U256::try_from_bytes_le(&hex::decode(value.start_amount)?)?,
            end_amount: U256::try_from_bytes_le(&hex::decode(value.end_amount)?)?,
        })
    }
}
