use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use json_rpc::{Request, Response};
use chess_lib::game::GameState;
use super::Rpc;

pub (super) fn proccess(mut socket: TcpStream, rpc: Arc<Rpc>) {
  // Spawn a new task to handle the connection
  let handle = tokio::spawn(async move {
    let mut buf = [0; 90000];

    while rpc.ctx.game.state != GameState::Ended {
      // Read data from the socket
      match socket.read(&mut buf).await {
        // Ok(n) if n == 0 => return, // Connection closed
        Ok(n) => {
          let request = std::str::from_utf8(&buf[0..n]).unwrap();
          let request_json = serde_json::from_str::<Request>(request).unwrap();
          let id = request_json.id;
          let name = request_json.method;
          let params = request_json.params;

          let response = rpc.call_method(id, name, params);
          let response_json = serde_json::to_string::<Response>(&response).unwrap();

          print!("Response json {:?}", response_json);

          // Write the data back to the socket
          if let Err(e) = socket.write_all(&response_json.as_bytes()).await {
              println!("Failed to write to socket; err = {:?}", e);
              // return;
          }

        }
        Err(e) => {
            println!("Failed to read from socket; err = {:?}", e);
            // return;
        }
      }
    }
    return 10;
  });
}
