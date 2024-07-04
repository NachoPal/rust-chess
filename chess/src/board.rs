use std::error::Error;
use std::fmt;
use std::collections::{HashMap, HashSet};
use super::pieces::{
  Piece, Color::{self, White, Black},
  King, Queen, Pawn, Rook, Bishop, Knight
};
use super::ensure;
use self::MovementKind::{Diagonal, Horizontal, Vertical, Knight as KnightMovement};
use self::Direction::{Forward, Backward, Left, Right, Unknown};

#[derive(Debug, PartialEq, Eq)]
pub enum MovementError {
	OutOfBounds,
	BlockedPath,
	IllegalMovement,
	Check,
  NoPiece,
  WrongCommand(String),
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
          MovementError::WrongCommand(command) => write!(f, "{:?} is not a valid movement command", command),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  Forward(u32),
  Backward(u32),
  Left(u32),
  Right(u32),
  Unknown,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MovementKind {
	Diagonal((Direction, Direction)),
	Horizontal(Direction),
	Vertical(Direction),
	Knight,
}

#[derive(Debug)]
pub struct Board<'a> {
  pub dimension: Position,
	pub positions: &'a mut HashMap<Position, Box<dyn Piece>>,
	pub pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
	pub pieces_dead: &'a mut HashMap<Color, Vec<Box<dyn Piece>>>
}

impl<'a> Board<'a> {
  pub fn new(
    dimension: Position,
    maybe_pieces: Option<Vec<(Position, Box<dyn Piece>)>>,
    positions: &'a mut HashMap<Position, Box<dyn Piece>>,
    pieces_set: &'a mut HashMap<Color, HashSet<Position>>,
    pieces_dead: &'a mut HashMap<Color, Vec<Box<dyn Piece>>>
  ) -> Self {
    let pieces = maybe_pieces.unwrap_or(vec![]);

    // TODO: Check pieces_set are not out of bounds

    // Initialize `pieces_set` and `piece_dead` in case either `White` and `Black` do not exist
    if pieces_set.get(&White).is_none() { pieces_set.insert(White, HashSet::new()); }
    if pieces_set.get(&Black).is_none() { pieces_set.insert(Black, HashSet::new()); }
    if pieces_dead.get(&White).is_none() { pieces_dead.insert(White, Vec::new()); }
    if pieces_dead.get(&Black).is_none() { pieces_dead.insert(Black, Vec::new()); }

    Self::do_add_pieces(positions, pieces_set, pieces);

    Board {
      dimension,
      positions,
      pieces_set,
      pieces_dead,
    }
  }

  pub fn add_pieces(&mut self, new_pieces: Vec<(Position, Box<dyn Piece>)>) {
    Self::do_add_pieces(self.positions, self.pieces_set, new_pieces);
  }

  fn do_add_pieces(
    positions: &mut HashMap<Position, Box<dyn Piece>>,
    pieces_set: &mut HashMap<Color, HashSet<Position>>,
    new_pieces: Vec<(Position, Box<dyn Piece>)>,
  ) {
    // TODO: Check new_pieces are not out of bounds
    // TODO: Check a piece does not exist already in that Position
    for (position, piece) in new_pieces {
      let piece_color = piece.color();
      if let Some(color) = pieces_set.get_mut(&piece_color) {
        color.insert(position);
      } else {
        pieces_set.insert(piece_color, HashSet::new());
        pieces_set.get_mut(&piece_color).expect("Color exists").insert(position);
      }
      positions.insert(position, piece);
    }
  }

  pub fn move_piece(&mut self, playing_color: Color, movement: &Movement) -> Result<(), MovementError> {
    self.can_move(playing_color, &movement).and_then(|_| {
      self.replace_square(movement)
    })
  }

  pub fn clean(&mut self) -> Result<(), MovementError> {
    let mut positions_to_remove = Vec::new();

    for color in [White, Black].iter() {
        if let Some(set) = self.pieces_set.get(color) {
            positions_to_remove.extend(set.iter().cloned());
            self.pieces_set.insert(*color, HashSet::new());
        }
        self.pieces_dead.insert(*color, Vec::new());
    }
    for position in positions_to_remove {
        self.remove_piece(&position)?;
    }

    Ok(())
  }

  fn can_move(&self, playing_color: Color, movement: &Movement) -> Result<bool, MovementError> {
    let piece = self.pick_piece(movement.from).ok_or(MovementError::NoPiece)?;
    let piece_color = piece.color();
    let movement_kind = self.movement_kind(playing_color, &movement)?;

    // Check if piece's color intented to be moved matches with color's turn
    ensure!(piece_color == playing_color, MovementError::WrongPiece(piece_color));
    // Check if the movement is valid for that piece
    self.is_valid_move(piece, &movement_kind)?;
    // Check it the movement target id valid
    ensure!(self.valid_target(playing_color, &movement), MovementError::BlockedPath);
    // Check if the movement path is blocked
    let blocked_path = self.blocked_path(&movement, &movement_kind);
    if !piece.is_king() && !piece.is_knight() {
      ensure!(!blocked_path, MovementError::BlockedPath);
    }
    // Check if valid Castle movement
    if piece.is_king() && blocked_path {
      // TODO: Check if valid Castle movement
    }
    // Check pawn special movements
    if piece.is_pawn() {
      // TODO: Check pawn special movements
      // - Forward(2) when `moved = false`
      // - kill piece
      // - special movement
    }
    Ok(true)
  }

  pub fn dead_pieces(&self, color: Color) -> &Vec<Box<dyn Piece>> {
    self.pieces_dead.get(&color).expect("Color exists")
  }

  fn is_valid_move(&self, piece: &Box<dyn Piece>, movement_kind: &MovementKind) -> Result<bool, MovementError> {
    let valid_moves = if piece.is_king() {
      vec![
        Vertical(Forward(1)),
        Vertical(Backward(1)),
        Horizontal(Left(1)),
        Horizontal(Right(1)),
        Diagonal((Forward(1), Right(1))),
        Diagonal((Forward(1), Left(1))),
        Diagonal((Backward(1), Right(1))),
        Diagonal((Backward(1), Left(1))),
      ]
    } else if piece.is_queen() {
      self.build_valid_moves(
        |y| { vec![
          Vertical(Forward(y)),
          Vertical(Backward(y)),
        ] },
        |x| { vec![
          Horizontal(Left(x)),
          Horizontal(Right(x)),
        ] },
        |z| { vec![
          Diagonal((Forward(z), Right(z))),
          Diagonal((Forward(z), Left(z))),
          Diagonal((Backward(z), Right(z))),
          Diagonal((Backward(z), Left(z))),
        ]} 
      )
    } else if piece.is_rook() {
      self.build_valid_moves(
        |y| { vec![
          Vertical(Forward(y)),
          Vertical(Backward(y)),
        ] },
        |x| { vec![
          Horizontal(Left(x)),
          Horizontal(Right(x)),
        ] },
        |_z| { vec![]} 
      )
    } else if piece.is_knight() {
      vec![KnightMovement]
    } else if piece.is_bishop() {
      self.build_valid_moves(
        |_y| { vec![] },
        |_x| { vec![] },
        |z| { vec![
          Diagonal((Forward(z), Right(z))),
          Diagonal((Forward(z), Left(z))),
          Diagonal((Backward(z), Right(z))),
          Diagonal((Backward(z), Left(z))),
        ]}
      )
    } else if piece.is_pawn() {
      vec![
        Vertical(Forward(1)),
        Vertical(Forward(2)),
        Diagonal((Forward(1), Left(1))),
        Diagonal((Forward(1), Right(1))),
      ]
    } else {
        vec![]
    };
    ensure!(valid_moves.iter().any(|m| m == movement_kind), MovementError::IllegalMovement);
    Ok(true)
  }

  fn build_valid_moves<Y, X, Z>(&self, fy: Y, fx: X, fz: Z) -> Vec<MovementKind>
    where 
      Y: Fn(u32) -> Vec<MovementKind>,
      X: Fn(u32) -> Vec<MovementKind>,
      Z: Fn(u32) -> Vec<MovementKind>,
  {
    let max_y = self.dimension.y as u32;
    let max_x = self.dimension.x as u32;
    let min = max_x.min(max_y);
    
    vec![
      (1..=max_y).into_iter().map(|y| fy(y)).flatten().collect::<Vec<MovementKind>>(),
      (1..=max_x).into_iter().map(|x| fx(x)).flatten().collect(),
      (1..=min).into_iter().map(|z| fz(z)).flatten().collect(),
    ].into_iter().flatten().collect()

  }

  fn pick_piece(&self, position: Position) -> Option<&Box<dyn Piece>> {
    self.positions.get(&position)
  }

  pub fn remove_piece(&mut self, position: &Position) -> Result<Box<dyn Piece>, MovementError> {
    let piece_origin = self.positions.remove(&position).ok_or(MovementError::NoPiece)?;
    self.pieces_set.get_mut(&piece_origin.color()).expect("Color exists").remove(&position);
    Ok(piece_origin)
  }

  #[cfg(test)]
  pub fn test_remove_piece(&mut self, position: &Position) -> Result<Box<dyn Piece>, MovementError> {
    self.remove_piece(position)
  }

  fn square_is_empty(&self, position: Position) -> bool {
    self.pick_piece(position).is_none()
  }

  fn valid_target(&self, playing_color: Color, movement: &Movement) -> bool {
    let target_square= self.pick_piece(movement.to);
    // Either a rival piece or empty
    let rival_piece = target_square.is_some_and(|piece| piece.color() != playing_color);
    let empty_square = target_square.is_none();

    rival_piece || empty_square
  }

  fn blocked_path(&self, movement: &Movement, movement_kind: &MovementKind) -> bool {
    match movement_kind {
      Vertical(direction) => {
        Self::path_range(direction, movement.from.y, movement.to.y).any(|y| {
          !self.square_is_empty(Position { x: movement.from.x, y })
        })
      },
      Horizontal(direction) => {
        Self::path_range(direction, movement.from.x, movement.to.x).any(|x| {
          !self.square_is_empty(Position { x, y: movement.from.y })
        })
      },
      Diagonal((vertical_directon, horizontal_direction)) => {
        println!("VERTICAL DIRECTION {:?}", vertical_directon);
        println!("HORIZONTAL DIRECTION {:?}", horizontal_direction);
        Self::path_range(vertical_directon, movement.from.y, movement.to.y).enumerate().any(|(i, y)| {
          // Self::path_range(horizontal_direction, movement.from.x, movement.to.x).any(|x| {
          //   let res = !self.square_is_empty(Position { x, y });
          //   println!("X: {:?}, Y: {:?}", x, y);
          //   res
          // })
          let x = Self::path_range(horizontal_direction, movement.from.x, movement.to.x).rev().collect::<Vec<i32>>()[i];
          println!("X: {:?}, Y: {:?}", x, y);
          !self.square_is_empty(Position { x, y })
        })
      },
      _ => false
    }
  }

  fn replace_square(&mut self, movement: &Movement) -> Result<(), MovementError> {
    // Remove piece from origin and update its `pieces_set`
    // let piece_origin = self.positions.remove(&movement.from).ok_or(MovementError::NoPiece)?;
    // self.pieces_set.get_mut(&piece_origin.color()).expect("Color exists").remove(&movement.from);
    let piece_origin = self.remove_piece(&movement.from)?;
    self.pieces_set.get_mut(&piece_origin.color()).expect("Color exists").insert(movement.to);

    // Insert origin piece in target and remove killed rival piece if existed in that square
    if let Some(killed_piece) = self.positions.insert(movement.to, piece_origin) {
      self.pieces_set.get_mut(&killed_piece.color()).expect("Color exists").remove(&movement.to);
      self.pieces_dead.get_mut(&killed_piece.color()).expect("Color exists").push(killed_piece);
    }

    Ok(())
  }

  fn movement_kind(&self, playing_color: Color, movement: &Movement) -> Result<MovementKind, MovementError> {
    // Check piece moves
    let no_move = movement.to.x == movement.from.x && movement.to.y == movement.from.y;
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
        return Ok(
          Diagonal(
            (
              Self::movement_direction(
                playing_color, movement, Vertical(Unknown), variance
              ),
              Self::movement_direction(
                playing_color, movement, Horizontal(Unknown), variance
              ),
            )
          )
        )
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
    movement: &Movement, 
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
      Vertical(_) => {
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

  fn path_range(direction: &Direction, from: i32, to: i32) -> std::ops::Range<i32> {
    match direction {
      // Forward(_) | Right(_)=> from + 1..to,
      // Backward(_) | Left(_) => to + 1..from,
      Forward(_) | Right(_)=> from + 1..to,
      Backward(_) | Left(_) => to + 1..from,
      _ => from..to
    }
  }
}
