use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SeaportMessageID {
  GetOrders = 0x01,
  Orders = 0x02,
  GetOrderHashes = 0x03,
  OrderHashes = 0x04,
  GetOrderCount = 0x05,
  OrderCount = 0x06,
  GetCriteria = 0x07,
  Criteria = 0x08,
}