use tokio::io::{AsyncReadExt, Error};

use crate::data_types::decodec::{Decodable, FixedSizeDecodable};

pub struct Ping {
    pub payload: i64,
}

impl Decodable for Ping {
    async fn decode<S: AsyncReadExt + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let payload = i64::fixed_decode(stream).await?;
        return Ok(Self { payload });
    }
}
