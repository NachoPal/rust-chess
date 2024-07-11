use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use json_rpc::{Request, Response};

fn ask_for_password() -> String {
  print!("Enter the game password: ");
  // // Flush the standard output to ensure the prompt is shown before reading input
  io::stdout().flush().unwrap();

  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Failed to read line");

  // Remove the newline character from the end of the input
  name.trim().to_string()
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    let password = ask_for_password();
    let method = "password".to_string();
    let params = vec![serde_json::json!(password)];

    let request = Request::new(method, params, None);

    // Write some data to the server
    let request_json = serde_json::to_string(&request).unwrap();
    stream.write_all(request_json.as_bytes()).await?;

    // Read the response from the server
    let mut buf = vec![0; 90000];
    let n = stream.read(&mut buf).await?;
    let response = serde_json::from_slice::<Response>(&buf[..n]).unwrap();

    match response {
      Response:: Success { result , .. } => {
        print!("{}", result.as_str().unwrap());
      }
      _ => {},
    }

    Ok(())
}
