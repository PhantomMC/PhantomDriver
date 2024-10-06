use std::sync::Arc;

use base64::{prelude::BASE64_STANDARD, Engine};
use minecrevy_text::Text;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt, Error};

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

#[derive(Clone)]
pub struct UuidPicker {
    uuids: Vec<String>,
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

pub async fn read_favicon_to_base64(mut file: impl AsyncReadExt + Unpin) -> Result<String, Error> {
    let mut buffer = Vec::new();
    file.read(&mut buffer).await?;
    return Ok(format!(
        "data:image/png;base64,{}",
        BASE64_STANDARD.encode(buffer)
    ));
}

impl UuidPicker {
    pub fn new() -> Self {
        return Self {
            uuids: serde_json::from_str(include_str!("valid_uuid.json")).unwrap(),
        };
    }

    pub fn next_uuid(self: &Arc<Self>) -> String {
        let mut random = SmallRng::from_entropy();
        let random_value = random.gen_range(0..self.uuids.len());
        return self.uuids.get(random_value).unwrap().clone();
    }
}

impl Players {
    pub fn compile(
        max_players: i32,
        players_online: i32,
        hover_text: Vec<String>,
        uuid_picker: Arc<UuidPicker>,
    ) -> Self {
        let player_sample: Vec<Player> = hover_text
            .iter()
            .map(|text| Player {
                name: text.clone(),
                id: uuid_picker.next_uuid(),
            })
            .collect();
        return Self {
            max: max_players,
            online: players_online,
            sample: player_sample,
        };
    }
}

impl Default for Status {
    fn default() -> Self {
        Self {
            version: Version {
                protocol: 0,
                name: String::from("Placeholder"),
            },
            players: Players {
                max: 0,
                online: 0,
                sample: Vec::new(),
            },
            description: Text::string("This is a placeholder"),
            favicon: Default::default(),
            enforces_secure_chat: Default::default(),
        }
    }
}
