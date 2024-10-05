use minecrevy_text::Text;
use serde::Serialize;
use tokio::io::{AsyncWrite, AsyncWriteExt, Error};

use crate::data_types::decodec::Encodable;

const PACKET_ID: u8 = 0x00;

#[derive(Serialize)]
pub struct Status {
    pub version: Version,
    pub players: Players,
    pub description: Text,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    async fn encode<S: AsyncWrite + Unpin>(self: &Self, stream: &mut S) -> Result<(), Error> {
        let mut payload = Vec::new();
        payload.write(&[PACKET_ID]).await?;
        let json = serde_json::to_string(self).unwrap();
        json.encode(&mut payload).await?;
        (payload.len() as i32).encode(stream).await?;
        return stream.write(&payload).await.map(|_| ());
    }
}
