use std::{io::ErrorKind, sync::Arc};

use clap::Parser;
use phantom_core::{
    data_types::decodec::{Decodable, Encodable, FixedSizeDecodable},
    request::{handshake::Handshake, login::Login, ping::Ping},
    response::{
        login_failure::LoginFailure,
        pong::Pong,
        status::{Status, UuidPicker},
    },
    sql::from_sql::FromSql,
};
use tokio::{
    io::{AsyncReadExt, Error, Take},
    net::{TcpListener, TcpStream},
};
use tokio_postgres::{Client, Config, NoTls};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Arguments {
    #[arg(short, long, default_value_t = 25565)]
    port: u16,

    #[arg(short, long)]
    addr: String,

    #[arg(long)]
    db_addr: String,

    #[arg(long)]
    db_port: u16,

    #[arg(long)]
    db_pass: String,

    #[arg(long)]
    db_user: String,

    #[arg(long)]
    db_name: String,
}

#[tokio::main]
pub async fn main() {
    let args = Arguments::parse();
    let mut db_config = Config::new();
    db_config
        .dbname(args.db_name)
        .host(args.db_addr)
        .port(args.db_port)
        .password(args.db_pass)
        .user(args.db_user);
    let (client, connection) = db_config.connect(NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    client
        .batch_execute(include_str!("./resources/init.sql"))
        .await
        .unwrap();
    let client_arc = Arc::new(client);
    let uuid_gen = Arc::new(UuidPicker::new());

    let listener = TcpListener::bind(format!("{}:{}", args.addr, args.port))
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
        let clone1 = client_arc.clone();
        let clone2 = uuid_gen.clone();
        tokio::task::spawn(async move {
            let result = handle_connection(stream.unwrap().0, clone1, clone2).await;
            if let Err(error) = result {
                println!("{error}")
            }
        });
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    client: Arc<Client>,
    uuid_gen: Arc<UuidPicker>,
) -> Result<(), Error> {
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
        handle_status(&mut stream, handshake, client, uuid_gen).await?
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
    client: Arc<Client>,
    uuid_gen: Arc<UuidPicker>,
) -> Result<(), Error> {
    let mut packet = take_packet(stream).await?;
    let packet_id = u8::fixed_decode(&mut packet).await?;
    if packet_id == 0x00 {
        let query = client
            .query(
                include_str!("resources/select_status.sql"),
                &[&handshake.server_address],
            )
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let status = match query.get(0) {
            Some(row) => Status::from_sql(row, handshake, uuid_gen).await?,
            None => Status::default(),
        };
        status.encode(stream).await?;
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
    client: Arc<Client>,
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
    let query = client
        .query(
            include_str!("./resources/select_disconnect.sql"),
            &[&handshake.server_address],
        )
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;
    let login_failure = match query.get(0) {
        Some(row) => LoginFailure::from_sql(row, handshake, login).await?,
        None => LoginFailure::default(),
    };
    return login_failure.encode(stream).await;
}
