use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use crate::stream::read::SharedStream;

pub trait SharedWrite {
  fn write_all_shared(&self, data: &Vec<u8>) -> Result<(), String>;
}

impl SharedWrite for SharedStream {
  fn write_all_shared(&self, data: &Vec<u8>) -> Result<(), String> {
    let mut stream = match self.lock() {
      Ok(guard) => guard,
      Err(e) => return Err(e.to_string())
    };
    
    return stream.write_all(data).map_err(|e| e.to_string());
  }
}