use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRequestPacket {
  pub e: Vec<u8>,
  pub n: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssignPacket {
  pub content: String,
  pub aes_key: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameRequestPacket {
  pub sender: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameResponsePacket {
  pub status: NameResponse
}
#[derive(Debug, Serialize, Deserialize)]
pub enum NameResponse {
  Success,
  Failure(String)
}