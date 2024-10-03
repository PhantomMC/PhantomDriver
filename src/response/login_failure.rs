use std::{collections::VecDeque, io::BufRead};

use minecrevy_text::Text;

use crate::data_types::decodec::Encodable;

pub struct LoginFailure {
    pub reason: Text,
}

impl Encodable for LoginFailure {
    fn encode<S: std::io::Write>(self: &Self, stream: &mut S) -> Result<(), std::io::Error> {
        let mut packet = VecDeque::new();
        0x00.encode(&mut packet)?;
        let reason = serde_json::to_string(&self.reason)?;
        println!("{reason}");
        reason.encode(&mut packet)?;
        (packet.len() as i32).encode(stream)?;
        stream.write(packet.fill_buf()?)?;
        return Ok(());
    }
}
