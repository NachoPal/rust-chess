use std::collections::HashMap;
use json_rpc_proc_macros::rpc;

mod listener;
use listener::tcp_listener;
mod proccess;
use proccess::proccess;
mod rpc;
use rpc::{Context, rpc};
use tokio::sync::broadcast::{self, Sender};

// Lib
use chess_lib::game::Game;
use chess_lib::board::{Board, Position};
use chess_lib::pieces::{Piece, Color};

const MAX_CHANNEL:  usize = 16;

pub fn clean_terminal() {
  print!("{esc}c", esc = 27 as char);
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

  let auth = HashMap::new();

  let (playing_color_tx, _) = broadcast::channel(MAX_CHANNEL);

  let ctx = Context {
    passwords,
    auth,
    game,
    playing_color_tx,
  };

  let rpc = rpc(ctx);

  let listener = tcp_listener().await?;

  println!("Waiting for connections...\n");

  loop {
    // Accept a new socket
    let (socket, addr) = listener.accept().await?;
    println!("- Established connection with {:?}", addr);
    proccess(socket, addr, rpc.clone());
  }

  Ok(())

  // TODO: add addr to RPC and check in auth if the addr is register as a valid addr
  // after password method registration
  // register_method should have a thrid parameter (bool) to tell `call_method` if it should
  // check for auth or not (all of methods will need but `password`)
}
