use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

async fn handle_connection(mut socket: TcpStream) -> eyre::Result<()> {
    loop {
        // read all the bytes
        let mut buf = Vec::new();
        socket.read_to_end(&mut buf).await?;

        // sent nothing? shutdown and close the connections
        if buf.is_empty() {
            break;
        }

        // write it back
        socket.write_all(&buf).await?;
    }

    Ok(())
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
