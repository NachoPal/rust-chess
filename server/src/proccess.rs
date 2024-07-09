use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use json_rpc::{Request};
use super::Rpc;

pub (super) fn proccess<'a , 'rpc>(mut socket: TcpStream, rpc: Arc<Rpc<'a, 'rpc>>) {
  // Spawn a new task to handle the connection
  let handle = tokio::spawn(async move {
      let mut buf = [0; 1024];

      // Read data from the socket
      match socket.read(&mut buf).await {
          // Ok(n) if n == 0 => return, // Connection closed
          Ok(n) => {
            // let message = std::str::from_utf8(&buf[..]).unwrap().to_string();
            let response = std::str::from_utf8(&buf[0..n]).unwrap();
            let message_json = serde_json::from_str::<Request>(response).unwrap();
            let name = message_json.method;
            let params = message_json.params;
            // println!("{:#?}", message_json);

              // let response = rpc.call_method(name, params);

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
      return 10;
  });
}
