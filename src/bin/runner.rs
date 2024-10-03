use std::{
    io::{Error, ErrorKind, Read, Take},
    net::{TcpListener, TcpStream},
};

use minecrevy_text::Text;
use phantom_core::{
    data_types::decodec::{Decodable, Encodable, FixedSizeDecodable},
    request::{handshake::Handshake, login::Login, ping::Ping},
    response::{
        login_failure::LoginFailure,
        pong::Pong,
        status::{Players, Status, Version},
    },
};

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").unwrap();
    for stream in listener.incoming() {
        if stream.is_err() {
            if stream.unwrap_err().kind() == ErrorKind::Interrupted {
                println!("Interupted!");
                return;
            }
            continue;
        }
        tokio::task::spawn(async move {
            let result = handle_connection(stream.unwrap()).await;
            if result.is_err() {
                let error = result.unwrap_err().to_string();
                println!("{error}")
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut packet = take_packet(&mut stream)?;
    let packet_id = u8::fixed_decode(&mut packet)?;
    if packet_id != 0x00 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("expected handshake first, got {packet_id}"),
        ));
    }
    let handshake = Handshake::decode(&mut packet)?;
    if handshake.next_state == 1 {
        handle_status(&mut stream, handshake)?
    } else if handshake.next_state == 2 {
        handle_login(&mut stream, handshake)?;
    }
    return Ok(());
}

fn take_packet(stream: &mut TcpStream) -> Result<Take<&mut TcpStream>, Error> {
    let number = i32::decode(stream)?;
    return Ok(stream.take(number as u64));
}

fn handle_status(stream: &mut TcpStream, handshake: Handshake) -> Result<(), Error> {
    let mut packet = take_packet(stream)?;
    let packet_id = u8::fixed_decode(&mut packet)?;
    if packet_id == 0x00 {
        let status_response = Status {
            version: Version {
                name: String::from("version text"),
                protocol: handshake.protocol_version,
            },
            players: Players {
                max: 1,
                online: 1,
                sample: vec![],
            },
            description: Text::string("Hello world"),
            enforces_secure_chat: true,
            favicon: Option::None,
        };
        status_response.encode(stream)?;
        let mut packet = take_packet(stream)?;
        let packet_id = u8::fixed_decode(&mut packet)?;
        if packet_id != 0x01 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "invalid packet id for ping request",
            ));
        }
        let ping = Ping::decode(&mut packet)?;
        Pong::from_ping(ping).encode(stream)?;
    } else if packet_id == 0x01 {
        let ping = Ping::decode(&mut packet)?;
        Pong::from_ping(ping).encode(stream)?;
    } else {
        return Err(Error::new(ErrorKind::InvalidData, "invalid packet id"));
    }
    return Ok(());
}

fn handle_login(stream: &mut TcpStream, handshake: Handshake) -> Result<(), Error> {
    let mut packet = take_packet(stream)?;
    let packet_id = u8::fixed_decode(&mut packet)?;
    if packet_id != 0x00 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "expected login start packet",
        ));
    }
    let login = Login::decode(&mut packet)?;
    let player_name = login.player_name;
    let login_failure = LoginFailure {
        reason: Text::string(format!("Hello {player_name}!")),
    };
    return login_failure.encode(stream);
}
