use std::io::ErrorKind;

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
use tokio::{
    io::{AsyncReadExt, Error, Take},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
pub async fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").await.unwrap();
    loop {
        let stream = listener.accept().await;
        if let Err(error) = stream {
            if error.kind() == ErrorKind::Interrupted {
                println!("Interupted!");
                return;
            }
            continue;
        }
        tokio::task::spawn(async move {
            if let Err(error) = handle_connection(stream.unwrap().0).await {
                println!("{error}")
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Error> {
    let mut packet = take_packet(&mut stream).await?;
    let packet_id = u8::fixed_decode(&mut packet).await?;
    if packet_id != 0x00 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("expected handshake first, got {packet_id}"),
        ));
    }
    let handshake = Handshake::decode(&mut packet).await?;
    if handshake.next_state == 1 {
        handle_status(&mut stream, handshake).await?
    } else if handshake.next_state == 2 {
        handle_login(&mut stream, handshake).await?;
    }
    return Ok(());
}

async fn take_packet(stream: &mut TcpStream) -> Result<Take<&mut TcpStream>, Error> {
    let number = i32::decode(stream).await?;
    return Ok(stream.take(number as u64));
}

async fn handle_status(stream: &mut TcpStream, handshake: Handshake) -> Result<(), Error> {
    let mut packet = take_packet(stream).await?;
    let packet_id = u8::fixed_decode(&mut packet).await?;
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
        status_response.encode(stream).await?;
        let mut packet = take_packet(stream).await?;
        let packet_id = u8::fixed_decode(&mut packet).await?;
        if packet_id != 0x01 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "invalid packet id for ping request",
            ));
        }
        let ping = Ping::decode(&mut packet).await?;
        Pong::from_ping(ping).encode(stream).await?;
    } else if packet_id == 0x01 {
        let ping = Ping::decode(&mut packet).await?;
        Pong::from_ping(ping).encode(stream).await?;
    } else {
        return Err(Error::new(ErrorKind::InvalidData, "invalid packet id"));
    }
    return Ok(());
}

async fn handle_login(stream: &mut TcpStream, handshake: Handshake) -> Result<(), Error> {
    let mut packet = take_packet(stream).await?;
    let packet_id = u8::fixed_decode(&mut packet).await?;
    if packet_id != 0x00 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "expected login start packet",
        ));
    }
    let login = Login::decode(&mut packet).await?;
    let player_name = login.player_name;
    let login_failure = LoginFailure {
        reason: Text::string(format!("Hello {player_name}!")),
    };
    return login_failure.encode(stream).await;
}
