mod game;
mod board;
mod pieces;
mod decl_macros;

use std::collections::HashMap;

use game::{Game, Player};
use board::{Board, Position, Movement};
use pieces::{Piece, Pawn, PieceFactory, Color::{Black, White}};

fn main() {
  let player_a = Player { name: "Nacho", color: White };
  let player_b = Player { name: "Pepe", color: Black };

  let dimension = Position { x: 7, y: 7 };
  let pieces: Option<Vec<(Position, ())>> = None;
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut board = Board::new(dimension, pieces, &mut positions, &mut pieces_set);

  let pawn = PieceFactory::create::<Pawn>(Black);
  let position = Position { x: 0, y: 0 };
  let pieces = vec![(position, pawn)];
  board.add_pieces(pieces);

  println!("This is my board {:?}", board);


  let mut game = Game::new(&mut board, (player_a, player_b));

  let movement = Movement { from: Position { x: 0, y: 0 }, to: Position { x: 0, y: 1 }};

  let result = game.move_piece(movement);

  println!("Movement result {:?}", result);
}
