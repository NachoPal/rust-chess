use std::fmt::Debug;
use super::board::{
  Movement, 
  MovementKind::{self, Horizontal,  Vertical, Diagonal, Knight},
  Direction::{self, Forward, Backward, Left, Right},
  Position
};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Color {
	Black,
	White,
}

pub trait Piece: Debug {
  fn new(color: Color) -> Self where Self: Sized;
  fn color(&self) -> Color { Color::White }
}

#[derive(Debug)]
pub struct Pawn(Color);

impl Pawn {
  fn new(color: Color) -> Self {
    Pawn(color)
  }
}

impl Piece for Pawn {
  fn new(color: Color) -> Self {
    Self::new(color)
  }
  fn color(&self) -> Color {
    self.0
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
