use crate::packet::PacketType;

pub trait Header {
  fn get_type(&self) -> Result<PacketType, String>;
  fn get_data_length(&self) -> usize;
  fn set_type(&mut self, packet_type: PacketType);
  fn set_data_length(&mut self, len: usize);
}

impl Header for Vec<u8> {
  fn get_type(&self) -> Result<PacketType, String> {
    return PacketType::try_from(self[0]);
  }

  fn get_data_length(&self) -> usize {
    let size = u16::from_be_bytes([self[1], self[2]]);
    return size as usize;
  }

  fn set_type(&mut self, packet_type: PacketType) {
    self[0] = packet_type as u8;
  }

  fn set_data_length(&mut self, len: usize) {
    let bytes: [u8; 2] = (len as u16).to_be_bytes();
    [self[1], self[2]] = bytes;
  }
}