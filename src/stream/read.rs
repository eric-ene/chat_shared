use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use crate::packet::PacketSymbols;


pub enum ReadError {
  Timeout,
  Other(String)
}

pub trait ReadUntil {
  fn read_until_char(&mut self, buf: &mut Vec<u8>, char: u8) -> Result<usize, String>;
  fn read_until_symbol(&mut self, buf: &mut Vec<u8>, sym: PacketSymbols) -> Result<usize, String>;
  fn read_until_timeout(&mut self, buf: &mut Vec<u8>, sym: PacketSymbols, timeout: Duration) -> Result<usize, ReadError>;
}

impl ReadUntil for TcpStream {
  fn read_until_char(&mut self, buf: &mut Vec<u8>, char: u8) -> Result<usize, String> {
    let mut reader = BufReader::new(self);
    return reader.read_until(char, buf).map_err(|e| e.to_string());
  }

  fn read_until_symbol(&mut self, buf: &mut Vec<u8>, sym: PacketSymbols) -> Result<usize, String> {
    let mut reader = BufReader::new(self);
    return reader.read_until(sym as u8, buf).map_err(|e| e.to_string());
  }

  fn read_until_timeout(&mut self, buf: &mut Vec<u8>, sym: PacketSymbols, timeout: Duration) -> Result<usize, ReadError> {
    let mut current_char = [0u8];
    let mut temp_buffer = Vec::new();
    
    let start = Instant::now();
    while start.elapsed() < timeout {
      let n_read = match self.read(&mut current_char) {
        Ok(n) => n,
        Err(e) => match e.kind() {
          ErrorKind::WouldBlock => 0,
          _ => {
            println!("{:?}", e);
            return Err(ReadError::Other(e.to_string()))
          }
        }
      };
      
      if n_read == 0 {
        continue;
      }
      
      temp_buffer.push(current_char[0]);
      
      if current_char[0] == sym as u8 {
        buf.append(&mut temp_buffer);
        return Ok(buf.len());
      }
    }
    
    buf.append(&mut temp_buffer);
    return Err(ReadError::Timeout);
  }
}