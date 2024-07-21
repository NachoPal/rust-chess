use std::io;
use tokio::net::TcpStream;
use json_rpc::Response;
use chess_lib::game::{Game, GameState};
use chess_server::ChessResponse;

mod rpc;
mod request;

use rpc::{movement, password, notify_turn};

use request::request;

pub fn clean_terminal() {
  print!("{esc}c\n", esc = 27 as char);
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
    let mut chess_response = serde_json::from_value::<ChessResponse>(result.0.clone()).unwrap();
    let player_color = chess_response.player_color.expect("there is player_color");
    let turn = chess_response.turn;
    let mut turn_color = Game::static_playing_color(turn);

    println!("Correct password, you are playing: {:?}\n", player_color);
    // print!("{}", board);
    print!("{}", chess_response.board);

    // Ask for movement
    loop {
      if chess_response.game_state == GameState::Ended { break }
      if player_color == turn_color {
        response = request(&mut stream, movement).await?;
      } else {
        response = request(&mut stream, || notify_turn(player_color)).await?;
      }

      clean_terminal();

      if response.is_success() {
        let result = response.result().expect("it is successful");
        chess_response = serde_json::from_value::<ChessResponse>(result.0.clone()).unwrap();
        turn_color = Game::static_playing_color(chess_response.turn);
      } else if response.is_error() {
        println!("{}", response);
      }

      print!("{}", chess_response.board);
    }

    Ok(())
}
