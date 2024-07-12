use std::io;

use json_rpc::{Request, Response};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn request<R: FnOnce() -> Request>(stream: &mut TcpStream, request: R) -> io::Result<Response> {
  // Write some data to the server
  let request_json = serde_json::to_string(&request()).unwrap();
  stream.write_all(request_json.as_bytes()).await?;

  // Read the response from the server
  let mut buf = vec![0; 90000];
  let n = stream.read(&mut buf).await?;
  stream.flush().await?;

  Ok(serde_json::from_slice::<Response>(&buf[..n]).unwrap())
}
