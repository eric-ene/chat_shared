use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePacket {
  pub receiver: String,
  pub content: Vec<u8>,
}