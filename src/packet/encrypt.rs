use crate::packet::{ProcessedPacket, HEADER_SIZE};
use eric_aes::{rsatools, aestools};
use eric_aes::aestools::CryptError;
use crate::packet::header::Header;

pub trait EncryptedPacket {
  fn new_raw_rsa(self, e: &Vec<u8>, n: &Vec<u8>) -> Vec<u8>;
  fn new_raw_aes(packet: ProcessedPacket, key: &Vec<u8>) -> Result<Vec<u8>, CryptError>;
}

impl EncryptedPacket for ProcessedPacket {
  fn new_raw_rsa(self, e: &Vec<u8>, n: &Vec<u8>) -> Vec<u8> {
    let base = ProcessedPacket::new_raw(self);
    let mut header = base[..HEADER_SIZE].to_vec();
    let body = base[HEADER_SIZE..].to_vec();

    let mut body_ciphertext = rsatools::encrypt_key(&body, &e, &n);

    // update header
    header.set_data_length(body_ciphertext.len());

    let mut retval = Vec::new();
    retval.append(&mut header);
    retval.append(&mut body_ciphertext);

    return retval;
  }

  fn new_raw_aes(packet: ProcessedPacket, key: &Vec<u8>) -> Result<Vec<u8>, CryptError> {
    let base = ProcessedPacket::new_raw(packet);
    let mut header = base[..HEADER_SIZE].to_vec();
    let body = base[HEADER_SIZE..].to_vec();

    let mut body_ciphertext = aestools::encrypt(&key, body)?;

    // update header
    header.set_data_length(body_ciphertext.len());

    let mut retval = Vec::new();
    retval.append(&mut header);
    retval.append(&mut body_ciphertext);

    return Ok(retval);
  }
}