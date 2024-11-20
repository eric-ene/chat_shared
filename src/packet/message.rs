use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePacket {
  pub sender: String,
  pub receiver: String,
  pub content: String,
}