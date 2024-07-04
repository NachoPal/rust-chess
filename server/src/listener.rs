use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn tcp_listener() -> Result<(), Box<dyn std::error::Error>> {
  // Bind the listener to the address
  let listener = TcpListener::bind("127.0.0.1:8080").await?;

  loop {
    // Accept a new socket
    let (mut socket, addr) = listener.accept().await?;
    println!("New connection from: {}", addr);

    // Spawn a new task to handle the connection
    tokio::spawn(async move {
        let mut buf = [0; 1024];

        // Read data from the socket
        match socket.read(&mut buf).await {
            Ok(n) if n == 0 => return, // Connection closed
            Ok(n) => {
                // Write the data back to the socket
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    println!("Failed to write to socket; err = {:?}", e);
                    // return;
                }
            }
            Err(e) => {
                println!("Failed to read from socket; err = {:?}", e);
                // return;
            }
        }
    });
  }
}
