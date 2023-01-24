use crate::structs::{DBConsideration, DBOffer, DBOrder};
use ssz_rs::prelude::*;
use thiserror::Error;

///! Seaport Gossip type definitions.
///
///! Basic building blocks for interacting with the P2P network.
///! See https://github.com/ProjectOpenSea/SIPs/blob/main/SIPS/sip-4.md#wire-protocol-messages

type H160 = Vector<u8, 20>;
type H256 = Vector<u8, 32>;
type Root = H256;
type Signature = Vector<u8, 65>;

/// The zero hash for extra data.
const ZERO_HASH: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";

/// Standard Seaport Gossip Network event format.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SeaportGossipsubEvent {
    /// The event type
    pub event: u8,
    /// The derived order hash
    pub order_hash: Root,
    /// The order itself
    pub order: Order,
    /// The current block number
    pub block_number: u64,
    /// The current block hash
    pub block_hash: Root,
}

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
    /// The offered items by the offerer
    pub offer: List<OfferItem, 100>,
    /// The items the offerer would like to obtain
    pub consideration: List<ConsiderationItem, 100>,
    /// The offerer address
    pub offerer: H160,
    /// The offerer signature
    pub signature: Signature,
    /// The order type
    pub order_type: u8,
    /// The start timestamp
    pub start_time: u32,
    /// The end timestamp for the order, which will help calculate price as time goes on.
    pub end_time: u32,
    /// The counter for the offer
    pub counter: U256,
    /// The salt of the order
    pub salt: U256,
    /// The key of the conduit used for approvals/transfers
    pub conduit_key: H256,
    /// A secondary account with special priviledges on the order
    pub zone: H160,
    /// The zone hash
    pub zone_hash: H256,
    /// The ID of the current chain
    pub chain_id: U256,
    /// Basic order types
    pub additional_recipients: List<H160, 50>,
    /// Advanced order types
    pub numerator: U256,
    /// The numerator
    pub denominator: U256,
    /// The denominator
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

/// Return a set of orders by their hashes after running a GetOrderHashes query.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct OrderHashes {
    #[serde(flatten)]
    req_id: u64,
    hashes: List<H256, 1_000_000>,
}

/// Request the list of items in a criteria hash. If the responding peer does not
/// have the criteria in its db, it should respond with an empty list.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct GetCriteria {
    #[serde(flatten)]
    req_id: u64,
    hash: H256,
}

/// Response to GetCriteria with the list of item IDs in the criteria. If the responding peer does not
/// have the criteria in its db, it should respond with an empty list.
#[derive(
    PartialEq, Eq, Debug, Default, Clone, SimpleSerialize, serde::Serialize, serde::Deserialize,
)]
#[serde(rename_all = "camelCase")]
pub struct CriteriaItems {
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
    /// Generic SSZ error.
    #[error("Ssz error")]
    SszError(String),
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

impl TryFrom<DBOrder> for Order {
    type Error = OfferConversionError;

    fn try_from(value: DBOrder) -> Result<Self, Self::Error> {
        let offer: List<OfferItem, 100> = match value
            .offers
            .iter()
            .map(|offer| offer.clone().try_into())
            .flatten()
            .collect::<Vec<OfferItem>>()
            .try_into()
        {
            Ok(offers) => offers,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        let consideration: List<ConsiderationItem, 100> = match value
            .considerations
            .iter()
            .map(|consideration| consideration.clone().try_into())
            .flatten()
            .collect::<Vec<ConsiderationItem>>()
            .try_into()
        {
            Ok(considerations) => considerations,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        let offerer: H160 = match hex::decode(value.offerer)?.try_into() {
            Ok(offerer) => offerer,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        let signature: Signature = match hex::decode(value.signature.as_bytes())?.try_into() {
            Ok(signature) => signature,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        let salt = U256::try_from_bytes_le(&hex::decode(value.salt.as_bytes())?)?;
        let conduit_key: H256 = match hex::decode(value.conduit_key.as_bytes())?.try_into() {
            Ok(v) => v,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        let zone: H160 = match hex::decode(value.zone.as_bytes())?.try_into() {
            Ok(v) => v,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        let zone_hash: H256 = match hex::decode(value.zone_hash.as_bytes())?.try_into() {
            Ok(v) => v,
            Err(e) => return Err(OfferConversionError::SszError(e.to_string())),
        };
        Ok(Order {
            offer,
            consideration,
            offerer,
            signature,
            salt,
            conduit_key,
            zone,
            zone_hash,
            extra_data: Vector::from_iter(vec![0u8; 32].clone()),
            order_type: value.order_type as u8,
            start_time: value.start_time as u32,
            end_time: value.end_time as u32,
            counter: U256::from(value.counter as u64),
            chain_id: 1.into(),
            additional_recipients: Default::default(),
            numerator: Default::default(),
            denominator: Default::default(),
        })
    }
}
