use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

async fn handle_async_tcp_client(mut socket: tokio::net::TcpStream) {
    let mut buffer = [0; 512];
    let bytes_read = socket.read(&mut buffer).await.unwrap();

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Received asynchronous TCP request: {}", request);

    let response = b"HTTP/1.1 200 OK\r\n\r\nHello, Asynchronous TCP!";
    socket.write_all(response).await.unwrap();
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_async_tcp_client(stream));
    }
}