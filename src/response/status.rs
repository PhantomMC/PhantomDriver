use std::{
    collections::VecDeque,
    io::{BufRead, Write},
};

use minecrevy_text::Text;
use serde::Serialize;

use crate::data_types::decodec::Encodable;

const PACKET_ID: u8 = 0x00;

#[derive(Serialize)]
pub struct Status {
    version: Version,
    players: Players,
    decription: Text,
    favicon: String,
    #[serde(rename = "enforcesSecureChat")]
    enforces_secure_chat: bool,
}

#[derive(Serialize)]
pub struct Version {
    name: String,
    protocol: i32,
}

#[derive(Serialize)]
pub struct Player {
    name: String,
    id: String,
}

#[derive(Serialize)]
pub struct Players {
    max: i32,
    online: i32,
    sample: Vec<Player>,
}

impl Encodable for Status {
    fn encode<S: std::io::Write>(self: &Self, stream: &mut S) -> Result<(), std::io::Error> {
        let mut payload = VecDeque::new();
        payload.write(&[PACKET_ID])?;
        serde_json::to_string(self).unwrap().encode(&mut payload)?;
        let payload_size = payload.len();
        (payload_size as i32).encode(stream)?;
        return stream.write_all(payload.fill_buf()?);
    }
}
