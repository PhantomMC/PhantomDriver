use std::io::{Error, Read};

use crate::data_types::decodec::Decodable;

pub struct Ping {
    pub identifier: i64,
}

impl Decodable for Ping {
    fn decode<S: Read>(stream: &mut S) -> Result<Self, Error> {
        let identifier = i64::decode(stream)?;
        return Ok(Self { identifier });
    }
}
