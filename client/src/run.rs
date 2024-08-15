//! Run module.
//!
//! Collection of Rpc `Request` to be submitted to the server
//!
use chess_lib::game::{Game, GameState};
use chess_server::ChessResponse;
use json_rpc::{Response, CONNECTION_CLOSED_BY_SERVER};
use std::sync::Arc;
use tokio::{
    io::{ReadHalf, WriteHalf},
    sync::Mutex,
};

use super::{movement, password, socket, TcpStream};

fn clean_terminal() {
    print!("\x1B[2J\x1B[H");
}

/// Ask for password to connect to server, running a loop afterwards in case of success
/// asking for movements chess movements
///
/// It will either wait for its color turn or wait for a movement input
///
/// It keeps printing an updated board returned by the server
pub async fn run(
    reader: Arc<Mutex<ReadHalf<TcpStream>>>,
    writer: &mut WriteHalf<TcpStream>,
) -> std::io::Result<String> {
    let mut response: Response;
    // Ask for password to play game until it is successful
    loop {
        response = socket::request(writer, reader.clone(), password).await?;
        clean_terminal();
        if response.is_success() {
            break;
        } else if response.is_error() {
            println!("{}", response);
        }
    }

    let result = response.result().expect("it is successful");
    let mut chess_response = serde_json::from_value::<ChessResponse>(result.0.clone()).unwrap();
    let player_color = chess_response.player_color.expect("there is player_color");
    let turn = chess_response.turn;
    let mut turn_color = Game::static_playing_color(turn);

    println!("Correct password, you are playing: {:?}\n", player_color);
    print!("{}", chess_response.board);

    // Ask for movement while the `Game` is not yet finished while waiting for server's response
    // in case it is its turn
    while chess_response.game_state != GameState::Ended {
        if player_color == turn_color {
            response = tokio::select! {
                    response = socket::read(reader.clone()) => {
                    response?
                },
                _ = socket::write(writer, movement) => {
                    socket::read(reader.clone()).await?
                },
            };
        } else {
            println!("\nIt is {:?} turn. Wait for his move...", !player_color);
            response = socket::read(reader.clone()).await?;
        }

        clean_terminal();

        if response.is_success() {
            let result = response.result().expect("it is successful");
            chess_response = serde_json::from_value::<ChessResponse>(result.0.clone()).unwrap();
            turn_color = Game::static_playing_color(chess_response.turn);

            print!("\n\n{}", chess_response.board);
        } else if response.is_error() {
            if let Response::Error { ref error, .. } = response {
                if error.code == CONNECTION_CLOSED_BY_SERVER {
                    // break;
                    return Ok(format!(
                        "Another connection has been established for {:?}",
                        player_color
                    ));
                }
                println!("{}", response);
                print!("{}", chess_response.board);
            }
        }
    }

    Ok("Game finished".to_string())
}
