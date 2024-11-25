use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignRequestPacket {}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssignPacket {
  pub content: String,
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