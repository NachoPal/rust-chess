use std::collections::{HashMap, HashSet};
use super::pieces::{Movement, MovementKind, Piece, Color, Color::{White, Black}};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Position {
	pub x: u32,
	pub y: u32,
}

pub struct Board<'a> {
	pub positions: &'a mut HashMap<Position, Box<dyn Piece>>,
	pub pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
}

impl<'a> Board<'a> {
  pub fn new(
    maybe_pieces: Option<Vec<(Position, impl Piece + 'static)>>,
    positions: &'a mut HashMap<Position, Box<dyn Piece>>,
    pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
  ) -> Self {
    let pieces = maybe_pieces.unwrap_or(vec![]);

    Self::_add_pieces(positions, pieces_set, pieces);

    Board {
      positions,
      pieces_set,
    }
  }

  pub fn add_pieces(&mut self, new_pieces: Vec<(Position, impl Piece + 'static)>) {
    Self::_add_pieces(self.positions, self.pieces_set, new_pieces);
  }

  fn _add_pieces(
    positions: &mut HashMap<Position, Box<dyn Piece>>,
    pieces_set: &mut HashMap<Color, HashSet<Position>>,
    new_pieces: Vec<(Position, impl Piece + 'static)>,
  ) {
    for (position, piece) in new_pieces {
      let piece_color = piece.color();
      if let Some(color) = pieces_set.get_mut(&piece_color) {
        color.insert(position);
      } else {
        pieces_set.insert(piece_color, HashSet::new());
      }
      positions.insert(position, Box::new(piece));
    }
  }
	//// Return Some(Position) if in `to` there is a rival piece.
	////  - a way of identifying the killed pieces and remove them from `Board.white` or `Board.black`
	//// It will call to `can_move`
	//// `color` is useful to know if the player is trying to move one of his pieces
	// fn move(&self, color: Color, movement: Movement) -> Result<Option<Position>, E>;
	//    // Check Piece at movement.from and see if its `valid_movements` includes the
	//    // result from movement.kind().
	// fn can_move(&self, color: Color, movement: Movement) -> Result<bool, E>;
}
