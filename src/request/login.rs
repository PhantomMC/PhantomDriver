use tokio::io::{AsyncRead, Error};

use crate::data_types::decodec::{Decodable, FixedSizeDecodable};

pub struct Login {
    pub player_name: String,
    pub player_uuid: Option<u128>,
}

impl Decodable for Login {
    async fn decode<S: AsyncRead + Unpin>(stream: &mut S) -> Result<Self, Error> {
        let player_name = String::decode(stream).await?;
        let player_uuid = u128::fixed_decode(stream).await;
        return Ok(Login {
            player_name,
            player_uuid: player_uuid.map_or(None, Some),
        });
    }
}
