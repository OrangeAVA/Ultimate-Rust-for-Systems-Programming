use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080...");

    while let Ok((mut socket, _)) = listener.accept().await {
        println!("Accepted a new connection");

        tokio::spawn(async move {
            let mut buffer = [0; 512];
            socket.read(&mut buffer).await.unwrap();

            // Convert the received bytes to a string for better readability
            let request = String::from_utf8_lossy(&buffer[..]);
            println!("Received request:\n{}", request);

            let response = b"HTTP/1.1 200 OK\r\n\r\nHello, Rust!";
            socket.write_all(response).await.unwrap();
            println!("Response sent.");
        });
    }
}