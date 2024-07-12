use std::collections::HashMap;
use std::sync::Arc;
use derive_proc_macros::rpc;

mod listener;
use listener::tcp_listener;
mod proccess;
use proccess::proccess;
mod rpc;
use rpc::rpc;

// Lib
use chess_lib::game::Game;
use chess_lib::board::{Board, Position};
use chess_lib::pieces::{Piece, Color};

pub fn clean_terminal() {
  print!("{esc}c", esc = 27 as char);
}

#[rpc]
struct Context {
  pub passwords: HashMap<String, Color>,
  pub game: Game,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  clean_terminal();
  let dimension = Position { x: 7, y: 7 };

  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  let board = Board::new(dimension, pieces);
  let mut game = Game::new(board);

  game.set_board();
  game.start();

  let mut passwords = HashMap::new();
  passwords.insert("white".to_string(), Color::White);
  passwords.insert("black".to_string(), Color::Black);

  let ctx = Context {
    passwords: passwords,
    game: game,
  };

  let rpc = rpc(Arc::new(ctx));

  let listener = tcp_listener().await?;

  loop {
    // Accept a new socket
    let (socket, _addr) = listener.accept().await?;
    proccess(socket, rpc.clone());
  }

  Ok(())
}
