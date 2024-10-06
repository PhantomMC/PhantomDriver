use std::{future::Future, sync::Arc};

use minecrevy_text::Text;
use tokio::io::{Error, ErrorKind};
use tokio_postgres::Row;

use crate::{
    request::{handshake::Handshake, login::Login},
    response::{
        login_failure::LoginFailure,
        status::{self, Players, Status, UuidPicker, Version},
    },
};

pub trait FromSql<T>: Sized {
    fn from_sql(
        row: &Row,
        handshake: Handshake,
        extra: T,
    ) -> impl Future<Output = Result<Self, Error>>;
}

impl FromSql<Arc<UuidPicker>> for Status {
    async fn from_sql(
        row: &Row,
        handshake: Handshake,
        extra: Arc<UuidPicker>,
    ) -> Result<Self, Error> {
        let version_name = row.get("version_name");
        let respond_outdated_protocol = row.get("protocol");
        let protocol;
        if respond_outdated_protocol {
            protocol = 0;
        } else {
            protocol = handshake.protocol_version;
        }

        let version = Version {
            protocol,
            name: version_name,
        };
        let hover_text = row.get("hover_text");
        let max_players = row.get("max_players");
        let players_online = row.get("players_online");
        let players = Players::compile(max_players, players_online, hover_text, extra);
        let favicon_bytes: &[u8] = row.get("favicon");
        let favicon = status::read_favicon_to_base64(favicon_bytes).await?;
        let enforces_secure_chat = row.get("enforce_secure_chat");
        let description: Text = serde_json::from_value(row.get("motd"))
            .map_err(|error| Error::new(ErrorKind::Other, error))?;

        return Ok(Self {
            version,
            players,
            description,
            favicon: Some(favicon),
            enforces_secure_chat,
        });
    }
}

impl FromSql<Login> for LoginFailure {
    async fn from_sql(row: &Row, handshake: Handshake, extra: Login) -> Result<Self, Error> {
        let reason: Text = serde_json::from_value(row.get("disconnect_msg"))
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        return Ok(Self { reason });
    }
}
