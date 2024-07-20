use std::collections::HashMap;
use std::sync::Arc;
use std::io::{self, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use json_rpc::{Rpc, Response, Request};

mod listener;
use listener::tcp_listener;
mod proccess;
use proccess::proccess;
mod rpc;
use rpc::{ChessContext, rpc};

// Lib
use chess_lib::game::{Game, GameState, Player};
use chess_lib::board::{Board, Movement, MovementError, Position};
use chess_lib::pieces::{Piece, Color::{self, Black, White}};

pub fn clean_terminal() {
  print!("{esc}c", esc = 27 as char);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  clean_terminal();
  let dimension = Position { x: 7, y: 7 };

  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut pieces_dead = HashMap::new();
  let mut board = Board::new(dimension, pieces, &mut positions, &mut pieces_set, &mut pieces_dead);
  let mut game = Game::new(&mut board);

  game.set_board();
  game.start();

  let mut passwords: HashMap<String, Color> = HashMap::new();
  passwords.insert("white".to_string(), Color::White);
  passwords.insert("black".to_string(), Color::Black);

  let ctx = ChessContext {
    passwords: &passwords,
    game: &game,
  };

  let rpc: Arc<Rpc<ChessContext>> = Arc::new(rpc(&ctx));

  let listener = tcp_listener().await?;

  loop {
    // Accept a new socket
    let (socket, _addr) = listener.accept().await?;
    proccess::<ChessContext>(socket, rpc.clone());
  }

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

  Ok(())
}
