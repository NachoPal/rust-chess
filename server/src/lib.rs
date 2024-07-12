use chess_lib::pieces::Color;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChessResponse {
  pub color: Color,
  pub turn: u32,
  pub board: String,
}
