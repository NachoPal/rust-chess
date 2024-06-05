use super::board::Board;
use super::pieces::Color;

pub struct Player<'a> {
	pub name: &'a str,
	pub color: Color,
}

pub enum GameState {
	Ready,
	OnGoing,
	Ended,
}

pub struct Game<'a> {
	pub board: &'a Board<'a>,
	pub state: GameState,
	pub players: (Player<'a>, Player<'a>),
	pub turn: u32,
}

impl<'a> Game<'a> {
  pub fn new(board: &'a Board, players: (Player<'a>, Player<'a>)) -> Self {
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
}
