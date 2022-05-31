// #![deny(missing_docs)]
//!An implementation of a key value store in Rust
use crate::error::{KvsError, Result};
use crate::utils::BUFFER_LENGTH;
use std::io::{Read, Write};
use std::net::TcpStream;

pub struct KvsClient {}

impl KvsClient {
    pub fn connect_and_send_request(ip_string: String, message: String) -> Result<String> {
        let mut stream = TcpStream::connect(ip_string)?;

        stream.write(message.as_bytes())?;

        let mut buffer = [0; BUFFER_LENGTH];

        stream.read(&mut buffer)?;

        let byte_vector: Vec<&[u8]> = buffer.split(|byte| &[*byte] == b"\n").collect();

        let content = byte_vector
            .get(0)
            .ok_or_else(|| KvsError::CommandError("Command unrecognized".to_string()))?;

        let string_response = String::from_utf8(content[..].to_vec())?;

        Ok(string_response)
    }
}
