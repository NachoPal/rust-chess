//! RPC module.
//!
//! Collection of Rpc `Response` to be returned by the server
//!
use chess_lib::{game::Game, pieces::Color};
use chess_server::ChessResponse;
use core::net::SocketAddr;
use json_rpc::{JsonRpcError, Response, CONNECTION_CLOSED_BY_SERVER, INVALID_PARAMS};
use json_rpc_proc_macros::{rpc, rpc_method};
use serde_json::Value;
use std::{collections::HashMap, sync::Arc};
use tokio::{
    io::WriteHalf,
    net::TcpStream,
    sync::{
        broadcast::{Receiver, Sender},
        Mutex,
    },
    time::{sleep, Duration},
};

use super::socket;

pub struct Authentication {
    pub addrs: HashMap<Color, SocketAddr>,
    pub new_addr_channel_tx: Sender<SocketAddr>,
}

#[rpc(auth = "password_authentication")]
pub struct Context {
    pub passwords: HashMap<String, Color>,
    pub auth: Authentication,
    pub game: Game,
    pub playing_color_tx: Sender<Color>,
}

/// Register a `SocketAddr` as a whitelisted address to submit requests on behalf
/// a certain playing `Color`
///
/// It will be called before each rpc method that requires authentication
#[allow(unused)]
async fn password_authentication(rpc: &Rpc<'_>, addr: SocketAddr) -> bool {
    let ctx = rpc.ctx.lock().await;
    let color_turn = Game::static_playing_color(ctx.game.turn);
    ctx.auth
        .addrs
        .get(&color_turn)
        .map_or(false, |valid_addr| *valid_addr == addr)
}

/// Checks the validity of a submitted password by a client
#[rpc_method]
pub async fn password(
    addr: SocketAddr,
    ctx_mutex: Arc<Mutex<Context>>,
    params: Params,
) -> Response {
    let mut ctx = ctx_mutex.lock().await;
    match params.first() {
        Some(Value::String(pasword)) => {
            if let Some(color) = ctx.passwords.get(pasword) {
                let color = color.to_owned();
                let chess_response = ChessResponse {
                    player_color: Some(color),
                    turn: ctx.game.turn,
                    board: ctx.game.print_board(color),
                    game_state: ctx.game.state,
                };

                if let Some(prev_addr) = ctx.auth.addrs.insert(color, addr) {
                    let _ = ctx.auth.new_addr_channel_tx.send(prev_addr);
                }

                Response::success(
                    serde_json::to_value::<ChessResponse>(chess_response).unwrap(),
                    None,
                )
            } else {
                let error = JsonRpcError {
                    code: 1,
                    message: "Incorrect password".to_string(),
                    data: None,
                };
                Response::error(error, None)
            }
        }
        _ => {
            let error = JsonRpcError {
                code: INVALID_PARAMS,
                message: "Invalid Params".to_string(),
                data: None,
            };
            Response::error(error, None)
        }
    }
}

/// Updates the board state based on a movement submitted by the client
#[rpc_method]
pub async fn movement(
    _addr: SocketAddr,
    ctx_mutex: Arc<Mutex<Context>>,
    params: Params,
) -> Response {
    let mut ctx = ctx_mutex.lock().await;
    match params.first() {
        Some(Value::String(movement)) => ctx
            .game
            .move_piece(movement.trim().to_string())
            .map(|_| {
                let playing_color = Game::static_playing_color(ctx.game.turn);

                let chess_response = ChessResponse {
                    player_color: None,
                    turn: ctx.game.turn,
                    board: ctx.game.print_board(!playing_color),
                    game_state: ctx.game.state,
                };

                let _ = ctx.playing_color_tx.send(playing_color).unwrap();
                let _ = ctx.playing_color_tx.send(!playing_color).unwrap();

                Response::success(
                    serde_json::to_value::<ChessResponse>(chess_response).unwrap(),
                    None,
                )
            })
            .unwrap_or_else(|err| {
                let error = JsonRpcError {
                    code: INVALID_PARAMS,
                    message: format!("{}", err),
                    data: None,
                };
                Response::error(error, None)
            }),
        _ => {
            let error = JsonRpcError {
                code: INVALID_PARAMS,
                message: "Invalid Params".to_string(),
                data: None,
            };
            Response::error(error, None)
        }
    }
}

/// Notify the client the opponent has moved a piece
pub async fn notify_turn(
    rpc: Arc<Rpc<'static>>,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
    response: Response,
) {
    match response {
        Response::Success { result, .. } => {
            let playing_color = serde_json::from_value::<ChessResponse>(result.clone())
                .unwrap()
                .player_color
                .expect("There is color");

            let mut rx: Receiver<Color>;

            // Mutex in its own scope to drop it after subscribing to the channel;
            // We want to avoid blocking the Mutex for other rpc calls
            // (`movement` method should be able to lock the Mutex and write into the channel)
            {
                let ctx = rpc.ctx.lock().await;
                rx = ctx.playing_color_tx.subscribe();
            }

            loop {
                // Waiting for `movement` to write into the channel to continue
                while rx.recv().await.unwrap() != playing_color {
                    sleep(Duration::from_secs(1)).await;
                }

                // Only when recived the expected turn color, we procced to lock the Mutex again
                let ctx = rpc.ctx.lock().await;

                let chess_response = ChessResponse {
                    player_color: None,
                    turn: ctx.game.turn,
                    board: ctx.game.print_board(playing_color),
                    game_state: ctx.game.state,
                };
                let response = Response::success(
                    serde_json::to_value::<ChessResponse>(chess_response).unwrap(),
                    None,
                );
                if let Err(e) = socket::write(Arc::clone(&writer), response).await {
                    println!("Failed to write to socket; err = {:?}", e);
                }
            }
        }
        _ => unreachable!(),
    }
}

/// Notify connection was closed by the server
pub async fn notify_close_connection(writer: Arc<Mutex<WriteHalf<TcpStream>>>) {
    let error = JsonRpcError {
        code: CONNECTION_CLOSED_BY_SERVER,
        message: "Connection closed by the server".to_string(),
        data: None,
    };
    let response = Response::error(error, None);

    if let Err(e) = socket::write(Arc::clone(&writer), response).await {
        println!("Failed to write to socket; err = {:?}", e);
    }
}

/// Create the RPC instance and register its methods
pub(super) fn rpc(ctx: Context) -> Arc<Rpc<'static>> {
    let mut rpc = Rpc::new(ctx);
    rpc.register_method("password".to_string(), password, false);
    rpc.register_method("movement".to_string(), movement, true);
    Arc::new(rpc)
}
