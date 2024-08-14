//! RPC module.
//!
//! Collection of Rpc `Request` to be submitted to the server
//!
use json_rpc::Request;
use std::io::{self, Write};

/// Prompt a user input with a message and return the value
fn ask_user(msg: String) -> String {
    print!("{}: ", msg);
    // Flush the standard output to ensure the prompt is shown before reading input
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    // Remove the newline character from the end of the input
    name.trim().to_string()
}

/// Build a `Request` for submitting a password
pub async fn password() -> Request {
    let password = ask_user("Enter game password".to_string());
    let method = "password".to_string();
    let params = vec![serde_json::json!(password)];

    Request::new(method, params, None)
}

/// Build a `Request` for submitting a piece movement
pub async fn movement() -> Request {
    let movement = ask_user("\nIt is your turn. Make your move".to_string());
    let method = "movement".to_string();
    let params = vec![serde_json::json!(movement)];

    Request::new(method, params, None)
}
