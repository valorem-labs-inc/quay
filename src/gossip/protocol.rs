use serde::{Deserialize, Serialize};

/// Seaport protocol message codes, indicating the action requested by a peer,
/// or sent by the local node.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeaportMessageID {
    /// GetOrders indicates that a peer has requested orders for a certain collection.
    GetOrders = 0x01,
    /// Orders means that the local node has received orders from a peer.
    Orders = 0x02,
    /// GetOrderHashes indicates that a peer has requested order hashes for a certain collection.
    GetOrderHashes = 0x03,
    /// OrderHashes indicates that the local node has received order hashes from a peer.
    OrderHashes = 0x04,
    /// GetOrderCount indicates that a peer has requested the amount of orders the local node has.
    GetOrderCount = 0x05,
    /// GetOrderCount indicates that the local node has received a count of how many orders the dialed peer has.
    OrderCount = 0x06,
    /// GetCriteria indicates that a peer has queried for some criteria for a certain order hash.
    GetCriteria = 0x07,
    /// Criteria indicates that the local node has received a criteria for a certain order hash.
    Criteria = 0x08,
}
