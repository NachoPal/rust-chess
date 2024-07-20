use std::any::Any;
use std::fmt::Debug;
use chess_proc_macros::Piece;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Color {
	Black,
	White,
}

pub trait Piece: Debug + AnyPiece + Send + Sync {
  fn new(color: Color) -> Self where Self: Sized;
  fn color(&self) -> Color { Color::White }
  fn is_king(&self) -> bool {
    self.as_any().downcast_ref::<King>().is_some()
  }
  fn is_queen(&self) -> bool {
    self.as_any().downcast_ref::<Queen>().is_some()
  }
  fn is_rook(&self) -> bool {
    self.as_any().downcast_ref::<Rook>().is_some()
  }
  fn is_bishop(&self) -> bool {
    self.as_any().downcast_ref::<Bishop>().is_some()
  }
  fn is_knight(&self) -> bool {
    self.as_any().downcast_ref::<Knight>().is_some()
  }
  fn is_pawn(&self) -> bool {
    self.as_any().downcast_ref::<Pawn>().is_some()
  }
  fn symbol(&self) -> char {
    let mut symbol = if self.is_king() {
      'K'
    } else if self.is_queen() {
      'Q'
    } else if self.is_rook() {
      'R'
    } else if self.is_bishop() {
      'B'
    } else if self.is_knight() {
      'N'
    } else if self.is_pawn() {
      'P' 
    } else { unreachable!() };

    if self.color() == Color::Black {
      let symbol_string = symbol.to_lowercase().collect::<String>();
      symbol = symbol_string.chars().next().unwrap();
    }
    symbol
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
