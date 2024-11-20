use serde::{Deserialize, Serialize};
use crate::packet::{PacketSymbols, PacketType};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPacket {
  pub content: String,
}