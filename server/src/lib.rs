use chess_lib::{game::{Game, GameState}, pieces::Color};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChessResponse {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub player_color: Option<Color>,
  pub turn: u32,
  pub board: String,
  pub game_state: GameState,
}
