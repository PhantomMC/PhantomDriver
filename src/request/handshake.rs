use std::io::Read;

use crate::data_types::decodec::Decodable;

pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl Decodable for Handshake {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, std::io::Error> {
        let protocol_version = i32::decode(stream)?;
        let server_address = String::decode(stream)?;
        let server_port = u16::decode(stream)?;
        let next_state = i32::decode(stream)?;

        return Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        });
    }
}
