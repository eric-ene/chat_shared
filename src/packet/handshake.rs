use serde::{Deserialize, Serialize};
use crate::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct HandshakePacket {
  pub status: HandshakeStatus,
  pub e: Vec<u8>,
  pub n: Vec<u8>,
  pub aes_key: Vec<u8>,
  pub src: User,
  pub dst: String,
}

#[derive(Debug, Deserialize, Serialize, Copy, Clone)]
pub enum HandshakeStatus {
  Request,
  Accept,
  Deny,
  NotFound,
  ServerError,
}
