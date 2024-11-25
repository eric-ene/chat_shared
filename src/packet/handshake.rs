use serde::{Deserialize, Serialize};
use crate::user::User;

#[derive(Debug, Deserialize, Serialize)]
pub struct HandshakePacket {
  pub status: HandshakeStatus,
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
