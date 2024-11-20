use serde::{Deserialize, Serialize};
use crate::packet::{PacketSymbols, PacketType, ProcessedPacket};

#[derive(Debug, Deserialize, Serialize)]
pub struct HandshakePacket {
  pub src: String,
  pub dst: String,
}
