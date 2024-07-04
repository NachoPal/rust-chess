use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use json_rpc::{Rpc, Request};

pub (super) fn proccess<Context: Send + Sync>(mut socket: TcpStream, rpc: Arc<Rpc<Context>>) {
  // Spawn a new task to handle the connection
  tokio::spawn(async move {
      let mut buf = [0; 1024];

      // Read data from the socket
      match socket.read(&mut buf).await {
          Ok(n) if n == 0 => return, // Connection closed
          Ok(n) => {
            // let message = std::str::from_utf8(&buf[..]).unwrap().to_string();
            let response = std::str::from_utf8(&buf[0..n]).unwrap();
            let message_json = serde_json::from_str::<Request>(response).unwrap();
            println!("{:#?}", message_json);

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
