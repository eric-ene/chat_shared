pub mod message;
pub mod handshake;
pub mod assign;
pub mod header;
pub mod encrypt;

use serde::{Deserialize, Serialize};
use eric_aes::{rsatools, aestools};
use eric_aes::aestools::CryptError;
use crate::packet::assign::{AssignPacket, AssignRequestPacket, NameRequestPacket, NameResponsePacket};
use crate::packet::handshake::HandshakePacket;
use crate::packet::header::Header;
use crate::packet::message::MessagePacket;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PacketType {
  NameAssign = 0xF0,
  Message = 0xF1,
  Handshake = 0xF2,
  NameAssignRequest = 0xF3,
  NameRequest = 0xF4,
  NameResponse = 0xF5,
}

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum PacketSymbols {
  Eom = 0x04,
}

pub const HEADER_SIZE: usize = 8;

#[derive(Debug)]
pub struct Packet {
  pub header: Vec<u8>, // THIS SHOULD ALWAYS HAVE A SIZE OF HEADER_SIZE
  pub data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessedPacket {
  Assign(AssignPacket),
  Message(MessagePacket),
  Handshake(HandshakePacket),
  AssignRequest(AssignRequestPacket),
  NameRequest(NameRequestPacket),
  NameResponse(NameResponsePacket),
}

impl From<&ProcessedPacket> for PacketType {
  fn from(value: &ProcessedPacket) -> Self {
    match value {
      ProcessedPacket::Assign(_) => PacketType::NameAssign,
      ProcessedPacket::Message(_) => PacketType::Message,
      ProcessedPacket::Handshake(_) => PacketType::Handshake,
      ProcessedPacket::AssignRequest(_) => PacketType::NameAssignRequest,
      ProcessedPacket::NameRequest(_) => PacketType::NameRequest,
      ProcessedPacket::NameResponse(_) => PacketType::NameResponse,
    }
  }
}


impl From<PacketType> for u8 {
  fn from(val: PacketType) -> Self {
    val as u8
  }
}

impl From<PacketSymbols> for u8 {
  fn from(val: PacketSymbols) -> Self {
    val as u8
  }
}

impl PartialEq for PacketType {
  fn eq(&self, other: &Self) -> bool {
    return *self as u8 == *other as u8;
  }
}

impl PartialEq<PacketType> for &ProcessedPacket {
  fn eq(&self, other: &PacketType) -> bool {
    return PacketType::from(*self) == *other;
  }
}

impl TryFrom<u8> for PacketType {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0xF0 => Ok(PacketType::NameAssign),
      0xF1 => Ok(PacketType::Message),
      0xF2 => Ok(PacketType::Handshake),
      0xF3 => Ok(PacketType::NameAssignRequest),
      0xF4 => Ok(PacketType::NameRequest),
      0xF5 => Ok(PacketType::NameResponse),
      val => Err(format!("Invalid packet type: {:#04X}", val)),
    }
  }
}

impl Packet {
  pub fn from_bytes(bytes: &mut Vec<u8>) -> Self {
    let header = bytes[0..HEADER_SIZE].to_vec();
    let data = bytes[HEADER_SIZE..bytes.len()].to_vec();
    
    *bytes = Vec::new();
    Self {
      header,
      data
    }
  }
  pub fn from_rsa_bytes(bytes: &Vec<u8>, d: &Vec<u8>, n: &Vec<u8>) -> Self {
    let header = bytes[0..HEADER_SIZE].to_vec();
    let data = bytes[HEADER_SIZE..bytes.len()].to_vec();
    
    let decrypted_data = rsatools::decrpyt_key(&data, &d, &n);
    
    Self {
      header,
      data: decrypted_data
    }
  }
  
  pub fn from_aes_bytes(bytes: &Vec<u8>, key: &Vec<u8>) -> Result<Self, CryptError> {
    let header = bytes[0..HEADER_SIZE].to_vec();
    let data = bytes[HEADER_SIZE..bytes.len()].to_vec();

    let decrypted_data = aestools::decrypt(&key, data)?;

    Ok(Self {
      header,
      data: decrypted_data
    })
  }

  pub fn process(&self) -> Result<ProcessedPacket, String> {
    return serde_json::from_slice::<ProcessedPacket>(&self.data).map_err(|err| err.to_string());
  }
}

impl ProcessedPacket {
  fn get_header(&self) -> [u8; HEADER_SIZE] {
    let mut header = [0u8; HEADER_SIZE];

    header[0] = PacketType::from(self) as u8;

    return header;
  }

  fn start_packet(&self) -> Vec<u8> {
    let header = self.get_header();
    return header.to_vec();
  }

  pub fn new_raw(packet: ProcessedPacket) -> Vec<u8> {
    let mut header = packet.start_packet(); // header

    let mut body = match serde_json::to_vec(&packet) {
      Ok(bytes) => bytes,
      Err(e) => {
        panic!(
          "Error serializing packet: {}\
          Packet contents: {:?}",
          e,
          packet
        );
      }
    };
    
    // pad to size that's a multiple of 16 bytes (128 bits) for AES
    let mut bytes_left = body.len() % 16;
    bytes_left = (16 - bytes_left) % 16;
    body.append(&mut vec![' ' as u8; bytes_left]);

    // finish setting up header
    header.set_data_length(body.len());
    
    let mut retval = Vec::new();
    retval.append(&mut header);
    retval.append(&mut body);
    
    return retval;
  }
}