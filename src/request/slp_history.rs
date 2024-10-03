use super::{handshake::Handshake, login::Login, ping::Ping};

pub struct SlpHistory {
    handshake: Option<Handshake>,
    ping: Option<Ping>,
    login: Option<Login>,
}

impl SlpHistory {
    pub fn new() -> SlpHistory {
        return Self {
            handshake: Option::None,
            ping: Option::None,
            login: Option::None,
        };
    }

    pub fn with_handshake(self: Self, handshake: Handshake) -> SlpHistory {
        return SlpHistory {
            handshake: Some(handshake),
            ping: self.ping,
            login: self.login,
        };
    }

    pub fn with_ping(self: Self, ping: Ping) -> SlpHistory {
        return SlpHistory {
            handshake: self.handshake,
            ping: Some(ping),
            login: self.login,
        };
    }

    pub fn with_login(self: Self, login: Login) -> SlpHistory {
        return SlpHistory {
            handshake: self.handshake,
            ping: self.ping,
            login: Some(login),
        };
    }

    pub fn merge(self: Self, other: SlpHistory) -> SlpHistory {
        return SlpHistory {
            handshake: self.handshake.or(other.handshake),
            ping: self.ping.or(other.ping),
            login: self.login.or(other.login),
        };
    }
}
