use std::any::Any;
use std::fmt::Debug;
use super::board::{
  Movement, 
  MovementKind::{self, Horizontal,  Vertical, Diagonal, Knight as KnightMovement},
  Direction::{self, Forward, Backward, Left, Right},
  Position
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Color {
	Black,
	White,
}

pub trait Piece: Debug + PieceExt {
  fn new(color: Color) -> Self where Self: Sized;
  fn color(&self) -> Color { Color::White }
  fn valid_moves(&self) -> Vec<MovementKind> { Vec::default() }
  fn is_knight(&self) -> bool {
    self.as_any().downcast_ref::<Knight>().is_some()
  }
  fn is_king(&self) -> bool {
    self.as_any().downcast_ref::<King>().is_some()
  }
}

// Extend the Piece trait to include a method to return &dyn Any
pub trait PieceExt {
  fn as_any(&self) -> &dyn Any;
}

impl<T: Piece + Any> PieceExt for T {
  fn as_any(&self) -> &dyn Any {
      self
  }
}

#[derive(Debug)]
pub struct King(Color);
#[derive(Debug)]
pub struct Queen(Color);
#[derive(Debug)]
pub struct Rook(Color);
#[derive(Debug)]
pub struct Bishop(Color);
#[derive(Debug)]
pub struct Knight(Color);
#[derive(Debug)]
pub struct Pawn(Color);

impl Piece for Pawn {
  fn new(color: Color) -> Self {
    // Self::new(color)
    Self(color)
  }
  fn color(&self) -> Color {
    self.0
  }
  fn valid_moves(&self) -> Vec<MovementKind> {
    vec![
      Vertical(Forward(1)),
      Vertical(Forward(2)),
      Diagonal((Forward(1), Right(1))),
      Diagonal((Forward(1), Left(1))),
    ]
  }
}

impl Piece for () {
  fn new(_color: Color) -> Self {
    ()
  }
}

pub struct PieceFactory;

impl PieceFactory {
  pub fn create<T: Piece>(color: Color) -> impl Piece {
    T::new(color)
  }
}
