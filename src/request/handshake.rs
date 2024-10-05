use tokio::io::AsyncReadExt;

use crate::data_types::decodec::{Decodable, FixedSizeDecodable};

pub struct Handshake {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32,
}

impl Decodable for Handshake {
    async fn decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, std::io::Error> {
        let protocol_version = i32::decode(stream).await?;
        let server_address = String::decode(stream).await?;
        let server_port = u16::fixed_decode(stream).await?;
        let next_state = i32::decode(stream).await?;

        return Ok(Self {
            protocol_version,
            server_address,
            server_port,
            next_state,
        });
    }
}
