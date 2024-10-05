use minecrevy_text::Text;
use tokio::io::{AsyncWriteExt, Error};

use crate::data_types::decodec::Encodable;

pub struct LoginFailure {
    pub reason: Text,
}

impl Encodable for LoginFailure {
    async fn encode<S: AsyncWriteExt + Unpin>(self: &Self, stream: &mut S) -> Result<(), Error> {
        let mut packet = Vec::new();
        0x00.encode(&mut packet).await?;
        let reason = serde_json::to_string(&self.reason)?;
        reason.encode(&mut packet).await?;
        (packet.len() as i32).encode(stream).await?;
        stream.write(&packet).await?;
        return Ok(());
    }
}
