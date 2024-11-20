pub mod message;
pub mod handshake;
pub mod assign;

use serde::{Deserialize, Serialize};
use crate::packet::assign::AssignPacket;
use crate::packet::handshake::HandshakePacket;
use crate::packet::message::MessagePacket;

#[repr(u8)]
pub enum PacketType {
  NameAssign = 0xF0,
  Message = 0xF1,
  Handshake = 0xF2,
}

#[repr(u8)]
pub enum PacketSymbols {
  Eof = 0x04,
}

#[derive(Debug)]
pub struct Packet {
  pub header: Vec<u8>, // THIS SHOULD ALWAYS HAVE A SIZE OF 8
  pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessedPacket {
  Assign(AssignPacket),
  Message(MessagePacket),
  Handshake(HandshakePacket),
}


impl From<PacketType> for u8 {
  fn from(val: PacketType) -> Self {
    val as u8
  }
}

impl TryFrom<u8> for PacketType {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0xF0 => Ok(PacketType::NameAssign),
      val => Err(format!("Invalid packet type: {:#04X}", val)),
    }
  }
}

impl Packet {
  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    let header = bytes[0..8].to_vec();
    let data = bytes[8..bytes.len()-1].to_vec();

    Self {
      header,
      data
    }
  }

  pub fn process(&self) -> Result<ProcessedPacket, String> {
    return serde_json::from_slice::<ProcessedPacket>(&self.data).map_err(|err| err.to_string());
  }
}

impl ProcessedPacket {
  fn get_header(&self) -> [u8; 8] {
    let mut header = [0u8; 8];

    match self {
      ProcessedPacket::Assign(_) => {
        header[0] = PacketType::NameAssign as u8
      }
      ProcessedPacket::Message(_) => {
        header[0] = PacketType::Message as u8
      }
      ProcessedPacket::Handshake(_) => {
        header[0] = PacketType::Handshake as u8
      }
    }

    return header;
  }

  fn start_packet(&self) -> Vec<u8> {
    let header = self.get_header();
    return header.to_vec();
  }

  pub fn new_raw(packet: ProcessedPacket) -> Vec<u8> {
    let mut raw_packet = packet.start_packet(); // header

    let bytes = match serde_json::to_vec(&packet) {
      Ok(bytes) => bytes,
      Err(e) => {
        panic!(
          "Error serializing packet: {}\
          Packet contents: {:?}",
          e,
          raw_packet
        );
      }
    };
    
    
    for byte in &bytes {
      raw_packet.push(*byte);
    }

    raw_packet.push(PacketSymbols::Eof as u8);
    return raw_packet;
  }
}