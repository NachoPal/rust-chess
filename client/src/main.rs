use std::io::{self, Write};
use tokio::net::TcpStream;
use json_rpc::{Request, Response};

mod rpc;
mod request;

use rpc::{password};

use request::request;

pub fn clean_terminal() {
  print!("{esc}c", esc = 27 as char);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Ask for password to play game
    let mut response: Response;
    loop {
      response = request(&mut stream, password).await?;
      clean_terminal();
      if response.is_success() {
        break;
      } else if response.is_error() {
        println!("{}", response);
      }
    }

    let result = response.result().expect("it is successful");
    let color = result.0.as_object().unwrap().get(&"color".to_string()).unwrap().as_str().unwrap();
    let board = result.0.as_object().unwrap().get(&"board".to_string()).unwrap().as_str().unwrap();
    println!("Correct password, you are playing: {}", color);
    print!("{}", board);

    Ok(())
}
