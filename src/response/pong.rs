use tokio::io::{AsyncWriteExt, Error};

use crate::{
    data_types::decodec::{Encodable, FixedSizeEncodable},
    request::ping::Ping,
};

pub struct Pong {
    payload: i64,
}

impl Encodable for Pong {
    async fn encode<S: AsyncWriteExt + Unpin>(self: &Self, stream: &mut S) -> Result<(), Error> {
        let mut packet = Vec::new();
        0x01.encode(&mut packet).await?;
        self.payload.fixed_encode(&mut packet).await?;
        let packet_length = packet.len();
        (packet_length as i32).encode(stream).await?;
        stream.write(&packet).await?;
        return Ok(());
    }
}

impl Pong {
    pub fn from_ping(ping: Ping) -> Self {
        return Self {
            payload: ping.payload,
        };
    }
}
