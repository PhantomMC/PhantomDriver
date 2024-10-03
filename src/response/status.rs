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
    pub version: Version,
    pub players: Players,
    pub decription: Text,
    pub favicon: Option<String>,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
}

#[derive(Serialize)]
pub struct Version {
    pub name: String,
    pub protocol: i32,
}

#[derive(Serialize)]
pub struct Player {
    pub name: String,
    pub id: String,
}

#[derive(Serialize)]
pub struct Players {
    pub max: i32,
    pub online: i32,
    pub sample: Vec<Player>,
}

impl Encodable for Status {
    fn encode<S: std::io::Write>(self: &Self, stream: &mut S) -> Result<(), std::io::Error> {
        let mut payload = VecDeque::new();
        payload.write(&[PACKET_ID])?;
        serde_json::to_string(self).unwrap().encode(&mut payload)?;
        let payload_size = payload.len();
        (payload_size as i32).encode(stream)?;
        return stream.write(payload.fill_buf()?).map(|_| ());
    }
}
