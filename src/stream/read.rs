use std::io::{BufRead, BufReader, ErrorKind, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::packet::{PacketSymbols, HEADER_SIZE};
use crate::packet::header::Header;

pub type SharedStream = Arc<Mutex<TcpStream>>;

pub enum ReadError {
  Timeout,
  Other(String)
}

pub trait ReadUntil {
  fn read_until_char(&mut self, buf: &mut Vec<u8>, char: u8) -> Result<usize, String>;
  fn read_until_symbol(&mut self, buf: &mut Vec<u8>, sym: PacketSymbols) -> Result<usize, String>;
  fn read_until_timeout(&mut self, buf: &mut Vec<u8>, sym: PacketSymbols, timeout: Duration) -> Result<usize, ReadError>;
}

pub trait ReadExact {
  fn read_exact_timeout(&self, buf: &mut Vec<u8>, len: usize, timeout: Duration) -> Result<(), String>;
  fn read_packet(&self) -> Result<Vec<u8>, String>;
}

impl ReadExact for SharedStream {
  /// UNDERLYING STREAM MUST BE NONBLOCKING
  fn read_exact_timeout(&self, buf: &mut Vec<u8>, len: usize, timeout: Duration) -> Result<(), String> {
    let start = Instant::now();

    let mut curr_buf = vec![0u8; len];
    let mut n_read = 0;
    while n_read < len && start.elapsed() < timeout {
      let slice = &mut curr_buf[n_read..len];
      
      let mut stream = self.lock().unwrap();
      
      n_read += match stream.read(slice) {
        Ok(n) => n,
        Err(err) => match err.kind() {
          ErrorKind::WouldBlock => 0,
          e => return Err(e.to_string())
        }
      };
    }
    
    buf.append(&mut curr_buf[..n_read].to_vec());
    return Ok(());
  }

  /// UNDERLYING STREAM MUST BE NONBLOCKING
  fn read_packet(&self) -> Result<Vec<u8>, String> {
    const TIMEOUT: Duration = Duration::from_millis(100);
    
    // read header
    let mut header = Vec::new();
    
    while header.len() < HEADER_SIZE {
      let len = HEADER_SIZE - header.len();
      match self.read_exact_timeout(&mut header, len, TIMEOUT) {
        Ok(_) => (),
        Err(e) => return Err(e)
      };
    }
    
    // read body
    let mut body = Vec::new();
    let message_size = header.get_data_length();
    
    while body.len() < message_size {
      let len = message_size - body.len();
      match self.read_exact_timeout(&mut body, len, TIMEOUT) {
        Ok(_) => (),
        Err(e) => return Err(e)
      }
    }
    
    // final packet
    let mut retval = Vec::new();
    retval.append(&mut header);
    retval.append(&mut body);
    
    return Ok(retval);
    
    unimplemented!()
  }
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