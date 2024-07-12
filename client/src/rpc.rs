use std::io::{self, Write};
use json_rpc::{Request, Response};

fn ask_user(msg: String) -> String {
  print!("{}: ", msg);
  // Flush the standard output to ensure the prompt is shown before reading input
  io::stdout().flush().unwrap();

  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Failed to read line");

  // Remove the newline character from the end of the input
  name.trim().to_string()
}

pub fn password() -> Request {
  let password = ask_user("Enter game password".to_string());
  let method = "password".to_string();
  let params = vec![serde_json::json!(password)];

  Request::new(method, params, None)
}

pub fn movement() -> Request {
  let movement = ask_user("Your movement".to_string());
  let method = "movement".to_string();
  let params = vec![serde_json::json!(movement)];

  Request::new(method, params, None)
}
