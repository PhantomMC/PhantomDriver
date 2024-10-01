use super::{handshake::Handshake, login::Login, ping::Ping};

pub struct SlpHistory {
    handshake: Option<Handshake>,
    ping: Option<Ping>,
    login: Option<Login>,
}
