use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write some data to the server
    let msg = b"Hello, world!";
    stream.write_all(msg).await?;
    println!("Sent: {:?}", msg);

    // Read the response from the server
    let mut buf = vec![0; 1024];
    let n = stream.read(&mut buf).await?;
    println!("Received: {:?}", &buf[..n]);

    Ok(())
}
