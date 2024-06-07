use std::error::Error;
use std::fmt;
use std::collections::{HashMap, HashSet};
use super::pieces::{Piece, Color::{self, White, Black}};
use super::ensure;
use self::MovementKind::{Diagonal, Horizontal, Vertical, Knight};
use self::Direction::{Forward, Backward, Left, Right, Unknown};

#[derive(Debug)]
pub enum MovementError {
	OutOfBounds,
	BlockedPath,
	IllegalMovement,
	Check,
  NoPiece,
  WrongPiece(Color),
}

impl fmt::Display for MovementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
          MovementError::OutOfBounds => write!(f, "Out of bounds"),
          MovementError::BlockedPath => write!(f, "Blocked path"),
          MovementError::IllegalMovement => write!(f, "Illegal movement"),
          MovementError::Check => write!(f, "That would be check"),
          MovementError::NoPiece => write!(f, "There is not any piece in that square"),
          MovementError::WrongPiece(color) => write!(f, "You can not play {:?} pieces", color),
        }
    }
}

impl Error for MovementError {}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Position {
	pub x: i32,
	pub y: i32,
}

#[derive(Debug)]
pub struct Movement {
	pub from: Position,
	pub to: Position,
}

#[derive(Debug)]
pub enum Direction {
  Forward(u32),
  Backward(u32),
  Left(u32),
  Right(u32),
  Unknown,
}

#[derive(Debug)]
pub enum MovementKind {
	Diagonal(Direction),
	Horizontal(Direction),
	Vertical(Direction),
	Knight,
}

#[derive(Debug)]
pub struct Board<'a> {
  pub dimension: Position,
	pub positions: &'a mut HashMap<Position, Box<dyn Piece>>,
	pub pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
}

impl<'a> Board<'a> {
  pub fn new(
    dimension: Position,
    maybe_pieces: Option<Vec<(Position, impl Piece + 'static)>>,
    positions: &'a mut HashMap<Position, Box<dyn Piece>>,
    pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
  ) -> Self {
    let pieces = maybe_pieces.unwrap_or(vec![]);

    Self::do_add_pieces(positions, pieces_set, pieces);

    Board {
      dimension,
      positions,
      pieces_set,
    }
  }

  pub fn add_pieces(&mut self, new_pieces: Vec<(Position, impl Piece + 'static)>) {
    Self::do_add_pieces(self.positions, self.pieces_set, new_pieces);
  }

  fn do_add_pieces(
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
        pieces_set.get_mut(&piece_color).expect("Color exists").insert(position);
      }
      positions.insert(position, Box::new(piece));
    }
  }

  pub fn move_piece(&mut self, playing_color: Color, movement: Movement) -> Result<(), MovementError> {
    self.can_move(playing_color, movement)
  }

  fn can_move(&self, playing_color: Color, movement: Movement) -> Result<(), MovementError> {
    let piece = self.pick_piece(movement.from).ok_or(MovementError::NoPiece)?;
    let movement_kind = self.movement_kind(playing_color, movement)?;

    Ok(())
  }

  fn pick_piece(&self, position: Position) -> Option<&Box<dyn Piece>> {
    self.positions.get(&position)
  }

  fn movement_kind(&self, playing_color: Color, movement: Movement) -> Result<MovementKind, MovementError> {
    // Check piece moves
    let no_move = movement.to.x == self.dimension.x && movement.to.y == self.dimension.y;
    ensure!(!no_move, MovementError::IllegalMovement);

    // Check it is not out of bounds
    let out_of_bounds = movement.to.x > self.dimension.x || movement.to.y > self.dimension.y;
    ensure!(!out_of_bounds, MovementError::OutOfBounds);

    let x_variance = movement.from.x.abs_diff(movement.to.x);
    let y_variance = movement.from.y.abs_diff(movement.to.y);
    let variance = (x_variance, y_variance);

    // Vertical movement
    if movement.from.x == movement.to.x {
      return Ok(Vertical(Self::movement_direction(playing_color, movement, Vertical(Unknown), variance)))
    }
    // Horizontal movement
    if movement.from.y == movement.to.y {
      return Ok(Horizontal(Self::movement_direction(playing_color, movement, Horizontal(Unknown), variance)))
    }
    // Diagonal && Knight movement
    if movement.from.y != movement.to.y && movement.from.x != movement.to.x {
      // Diagonal
      if x_variance == y_variance {
        return Ok(Diagonal(Self::movement_direction(playing_color, movement, Diagonal(Unknown), variance)))
      }
      // Knight
      if (x_variance == 2 && y_variance == 1) || (x_variance == 1 && y_variance == 2) {
        return Ok(MovementKind::Knight);
      }
    }
    
    Err(MovementError::IllegalMovement)
  }

  fn movement_direction(
    playing_color: Color, 
    movement: Movement, 
    movement_kind: MovementKind,
    (x_variance, y_variance): (u32, u32),
  ) -> Direction {
    match movement_kind {
      Horizontal(_) => {
        if movement.to.x > movement.from.x {
          Right(x_variance)
        } else if movement.to.x < movement.from.x {
          Left(x_variance)
        } else { Unknown }
      },
      Vertical(_) | Diagonal(_) => {
        if movement.to.y > movement.from.y {
          if playing_color == White {
            Forward(y_variance)
          } else {
            Backward(y_variance)
          }
        } else if movement.to.y < movement.from.y {
          if playing_color == Black {
            Forward(y_variance)
          } else {
            Backward(y_variance)
          }
        } else { Unknown }
      },
      _ => Unknown
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
