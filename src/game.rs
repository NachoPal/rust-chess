use crate::board::{Movement, MovementError, Position};
use super::board::Board;
use super::pieces::{Color::{self, Black, White}, Piece};

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
