use std::{fmt::format, io::ErrorKind};

use clap::Parser;
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
use tokio_postgres::{Client, Config, GenericClient, NoTls};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = 25565)]
    port: u16,

    #[arg(short, long)]
    address: String,

    #[arg(long)]
    db_addr: String,

    #[arg(long)]
    db_port: u16,

    #[arg(long)]
    db_pass: String,
}

#[tokio::main]
pub async fn main() {
    let args = Arguments::parse();
    let mut db_config = Config::new();
    db_config
        .dbname(args.db_addr)
        .port(args.port)
        .password(args.db_pass);
    let (client, connection) = db_config.connect(NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client.batch_execute(include_str!("./resources/init.sql"));

    let listener = TcpListener::bind(format!("{}:{}", args.address, args.port))
        .await
        .unwrap();
    loop {
        let stream = listener.accept().await;
        if let Err(error) = stream {
            if error.kind() == ErrorKind::Interrupted {
                println!("Interupted!");
                return;
            }
            continue;
        }
        tokio::task::spawn(handle_connection(stream.unwrap().0, &client));
    }
}

async fn handle_connection(mut stream: TcpStream, client: &Client) -> Result<(), Error> {
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
        handle_status(&mut stream, handshake, client).await?
    } else if handshake.next_state == 2 {
        handle_login(&mut stream, handshake, client).await?;
    }
    return Ok(());
}

async fn take_packet(stream: &mut TcpStream) -> Result<Take<&mut TcpStream>, Error> {
    let number = i32::decode(stream).await?;
    return Ok(stream.take(number as u64));
}

async fn handle_status(
    stream: &mut TcpStream,
    handshake: Handshake,
    client: &Client,
) -> Result<(), Error> {
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

async fn handle_login(
    stream: &mut TcpStream,
    handshake: Handshake,
    client: &Client,
) -> Result<(), Error> {
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
