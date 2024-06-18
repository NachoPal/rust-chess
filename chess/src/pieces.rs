use std::any::Any;
use std::fmt::Debug;
use derive_proc_macros::Piece;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum Color {
	Black,
	White,
}

pub trait Piece: Debug + AnyPiece {
  fn new(color: Color) -> Self where Self: Sized;
  fn color(&self) -> Color { Color::White }
  fn is_knight(&self) -> bool {
    self.as_any().downcast_ref::<Knight>().is_some()
  }
  fn is_king(&self) -> bool {
    self.as_any().downcast_ref::<King>().is_some()
  }
  fn is_pawn(&self) -> bool {
    self.as_any().downcast_ref::<Pawn>().is_some()
  }
}

// Extend the Piece trait to include a method to return &dyn Any
pub trait AnyPiece {
  fn as_any(&self) -> &dyn Any;
}

impl<T: Piece + Any> AnyPiece for T {
  fn as_any(&self) -> &dyn Any {
      self
  }
}

#[derive(Debug, Piece)]
pub struct King(Color);

#[derive(Debug, Piece)]
pub struct Queen(Color);

#[derive(Debug, Piece)]
pub struct Rook(Color);

#[derive(Debug, Piece)]
pub struct Bishop(Color);

#[derive(Debug, Piece)]
pub struct Knight(Color);

#[derive(Debug, Piece)]
pub struct Pawn(Color);

pub struct PieceFactory;

impl PieceFactory {
  pub fn create<T: Piece + 'static>(color: Color) -> Box<dyn Piece> {
    Box::new(T::new(color))
  }
}
