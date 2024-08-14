use chess_lib::{game::GameState, pieces::Color};
use serde::{Deserialize, Serialize};

/// Response returned by the server
#[derive(Serialize, Deserialize, Debug)]
pub struct ChessResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_color: Option<Color>,
    pub turn: u32,
    pub board: String,
    pub game_state: GameState,
}
