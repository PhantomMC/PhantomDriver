use std::{collections::VecDeque, io::BufRead};

use crate::{
    data_types::decodec::{Encodable, FixedSizeEncodable},
    request::ping::Ping,
};

pub struct Pong {
    payload: i64,
}

impl Encodable for Pong {
    fn encode<S: std::io::Write>(self: &Self, stream: &mut S) -> Result<(), std::io::Error> {
        let mut packet = VecDeque::new();
        0x01.encode(&mut packet)?;
        self.payload.fixed_encode(&mut packet)?;
        let packet_length = packet.len();
        (packet_length as i32).encode(stream)?;
        stream.write(packet.fill_buf()?)?;
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
