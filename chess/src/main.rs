use std::collections::HashMap;
use chess::game::{Game, Player};
use chess::board::{Board, Position, Movement};
use chess::pieces::{Piece, Color::{Black, White}};

fn main() {
  let player_a = Player { name: "Nacho", color: White };
  let player_b = Player { name: "Pepe", color: Black };

  let dimension = Position { x: 7, y: 7 };

  let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut board = Board::new(dimension, pieces, &mut positions, &mut pieces_set);

  let mut game = Game::new(&mut board, (player_a, player_b));

  game.set_board();

  let movement = Movement { from: Position { x: 0, y: 0 }, to: Position { x: 5, y: 0 }};

  let result = game.move_piece(movement);

  println!("Movement result {:?}", game.board.positions);
}
