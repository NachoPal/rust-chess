use std::collections::HashMap;
use std::io::{self, Write};
use tokio::net::TcpListener;

mod listener;
use listener::tcp_listener;

// Lib
use chess_lib::game::{Game, GameState, Player};
use chess_lib::board::{Board, Movement, MovementError, Position};
use chess_lib::pieces::{Piece, Color::{self, Black, White}};

pub fn clean_terminal() {
  print!("{esc}c", esc = 27 as char);
}

pub fn ask_player_name(color: Color) -> String {
  print!("Enter the name of the {:?} player: ", color);
  // Flush the standard output to ensure the prompt is shown before reading input
  io::stdout().flush().unwrap();

  let mut name = String::new();
  io::stdin().read_line(&mut name).expect("Failed to read line");
  
  // Remove the newline character from the end of the input
  name.trim().to_string()
}
#[tokio::main]
async fn main() {
  clean_terminal();
  let dimension = Position { x: 7, y: 7 };

  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();
  let mut board = Board::new(dimension, pieces, &mut positions, &mut pieces_set, &mut pieces_dead);

  let white_name = ask_player_name(White);
  let black_name = ask_player_name(Black);
  let white_player = Player { name: white_name.as_str(), color: White };
  let black_player = Player { name: black_name.as_str(), color: Black };

  let mut game = Game::new(&mut board, (white_player, black_player));

  game.set_board();
  game.start();

  // let _ = tcp_listener().await;

  while game.is_ongoing() {
    // clean_terminal();
    print!("{}", game.print_board());
    match game.ask_movement() {
      Err(e) => println!("Error: {}", e),
      Ok(movement) => {
        println!("MOVEMENT {:?}", movement);
        match game.move_piece(movement) {
          Err(e) => eprintln!("Error: {}", e),
          Ok(_)=> game.new_turn(),
        }
      }
    }
  }

  // Ok(())

  // let movement = Movement { from: Position { x: 0, y: 0 }, to: Position { x: 5, y: 0 }};

  // let result = game.move_piece(movement);

  // println!("Movement result {:?}", game.board.positions);
}
