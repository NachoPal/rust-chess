mod game;
mod board;
mod pieces;

use std::collections::HashMap;

use game::{Game, Player};
use board::{Board, Position};
use pieces::{Piece, Pawn, PieceFactory, Color::{Black, White}};

fn main() {
  let player_a = Player { name: "Nacho", color: White };
  let player_b = Player { name: "Pepe", color: Black };

  let pieces: Option<Vec<(Position, ())>> = None;
  let mut positions = HashMap::new();
  let mut pieces_set = HashMap::new();
  let mut board = Board::new(pieces, &mut positions, &mut pieces_set);

  let pawn = PieceFactory::create::<Pawn>(White);
  let position = Position { x: 0, y: 0 };
  let pieces = vec![(position, pawn)];
  board.add_pieces(pieces);


  let mut game = Game::new(&board, (player_a, player_b));
}
