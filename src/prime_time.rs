use serde::{Deserialize, Serialize};
use serde_json::Number;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

#[derive(Debug, Deserialize)]
struct Request {
    method: String,
    number: Number,
}

impl Request {
    fn is_valid(&self) -> bool {
        self.method == "isPrime"
    }

    fn is_prime(&self) -> bool {
        if !self.number.is_u64() {
            return false;
        }

        // SAFETY: we just checked
        is_prime::is_prime(&format!("{}", self.number.as_u64().unwrap()))
    }
}

#[derive(Debug, Serialize)]
struct Response {
    method: String,
    prime: bool,
}

impl Response {
    fn new_valid(prime: bool) -> Self {
        Self {
            method: String::from("isPrime"),
            prime,
        }
    }

    fn new_invalid() -> Self {
        Self {
            method: String::from("isNotPrime"),
            prime: false,
        }
    }
}

async fn read(socket: &mut BufReader<OwnedReadHalf>) -> eyre::Result<Request> {
    let mut buf = String::new();
    socket.read_line(&mut buf).await?;

    Ok(serde_json::from_str(&buf)?)
}

async fn write(socket: &mut OwnedWriteHalf, payload: &Response) -> eyre::Result<()> {
    let body = serde_json::to_vec(payload)?;
    socket.write_all(&body).await?;
    socket.write(b"\n").await?;

    Ok(())
}

async fn handle_connection(socket: TcpStream) -> eyre::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);

    loop {
        match read(&mut reader).await {
            Ok(request) if request.is_valid() => {
                let response = Response::new_valid(request.is_prime());
                write(&mut writer, &response).await?;
            }
            _ => {
                write(&mut writer, &Response::new_invalid()).await?;
            }
        }
    }
}

pub async fn main(host: String, port: u16) -> eyre::Result<()> {
    let server = tokio::net::TcpListener::bind((host.as_ref(), port)).await?;
    tracing::info!("listening to {host}:{port}");

    loop {
        let (socket, addr) = server.accept().await?;
        tracing::info!("accepted connection from {addr}");
        tokio::spawn(async move { handle_connection(socket).await });
    }
}
