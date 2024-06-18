use crate::pieces::Knight;

use super::board::{Board, Movement, MovementError, Position};
use super::pieces::{
  Color::{self, Black, White}, Piece, PieceFactory,
  Pawn, King, Queen, Rook, Bishop
};

pub struct Player<'a> {
	pub name: &'a str,
	pub color: Color,
}

pub enum GameState {
	Ready,
	OnGoing,
  Paused,
	Ended,
}

pub struct Game<'a> {
	pub board: &'a mut Board<'a>,
	pub state: GameState,
	pub players: (Player<'a>, Player<'a>),
	pub turn: u32,
}

impl<'a> Game<'a> {
  pub fn new(board: &'a mut Board<'a>, players: (Player<'a>, Player<'a>)) -> Self {
    Game {
      board,
      state: GameState::Ready,
      players,
      turn: 0,
    }
  }

  pub fn set_board(&mut self) {
    let mut pieces = self.build_pieces(White);
    pieces.extend(self.build_pieces(Black));
    self.board.add_pieces(pieces)
  }

  fn build_pieces(&mut self, color: Color) -> Vec<(Position, Box<dyn Piece>)> {
    let (row, offset) = match color {
      White => (0, 1),
      Black => (self.board.dimension.y - 1, -1),
    };
    //King
    let king = (Position { x: 4, y: row }, PieceFactory::create::<King>(color));
    //King
    let queen = (Position { x: 3, y: row }, PieceFactory::create::<Queen>(color));
    // Rooks
    let rook_left = (Position { x: 0, y: row }, PieceFactory::create::<Rook>(color));
    let rook_right = (Position { x: 7, y: row }, PieceFactory::create::<Rook>(color));
    // Knights
    let knight_left = (Position { x: 1, y: row }, PieceFactory::create::<Knight>(color));
    let knight_right = (Position { x: 6, y: row }, PieceFactory::create::<Knight>(color));
    // Bishops
    let bishop_left = (Position { x: 2, y: row }, PieceFactory::create::<Bishop>(color));
    let bishop_right = (Position { x: 5, y: row }, PieceFactory::create::<Bishop>(color));

    let mut pieces = vec![king, queen, rook_left, rook_right, knight_left, knight_right, bishop_left, bishop_right];
    
    // Paws
    for x in 0..self.board.dimension.x {
      let pawn = (Position { x, y: row + offset }, PieceFactory::create::<Pawn>(color));
      pieces.push(pawn)
    }
    pieces
  }

  fn start(&mut self) {
    self.state = GameState::OnGoing;
  }

  fn end(&mut self) {
    self.state = GameState::Ended;
  }

  fn new_turn(&mut self) {
    self.turn+=1;
  }

  pub fn move_piece(&mut self, movement: Movement) -> Result<(), MovementError> {
    self.board.move_piece(self.playing_color(), movement)
  }

  fn playing_color(&self) -> Color {
    if self.turn%2 == 0 { White } else { Black }
  }
}
