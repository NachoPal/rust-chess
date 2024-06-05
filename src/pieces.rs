use super::board::Position;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum Color {
	Black,
	White,
}

pub struct Movement {
	from: Position,
	to: Position,
}

pub enum MovementKind {
	Diagonal,
	DiagonalOne,
	Horizontal,
	HorzonatalOne,
	Vertical,
	VerticalOne,
	Knight,
	Pawn,
}

pub trait Piece {
  fn new(color: Color) -> Self where Self: Sized;
	fn valid_moves(&self) -> Vec<MovementKind> { Vec::new() }
  fn color(&self) -> Color { Color::White }
}

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
  fn valid_moves(&self) -> Vec<MovementKind> {
      vec![MovementKind::Pawn]
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
