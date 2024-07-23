use json_rpc::{JsonRpcError, Response, INVALID_PARAMS};
use serde_json::Value;
// use super::{Rpc, Context};
use std::sync::Arc;
use tokio::{sync::{broadcast::Receiver, Mutex}, time::{sleep, Duration}};
use chess_server::ChessResponse;
use chess_lib::{pieces::Color, game::Game};
use json_rpc_proc_macros::{rpc, rpc_method};
use std::collections::HashMap;
use tokio::sync::broadcast::{self, Sender};
use core::net::SocketAddr;

#[rpc(auth = "password_authentication")]
pub struct Context {
  pub passwords: HashMap<String, Color>,
  pub auth: HashMap<Color, SocketAddr>,
  pub game: Game,
  pub playing_color_tx: Sender<Color>, 
}

#[allow(unused)]
async fn password_authentication(rpc: &Rpc<'_>, addr: SocketAddr) -> bool {
  let ctx = rpc.ctx.lock().await;
  let color_turn = Game::static_playing_color(ctx.game.turn);
  ctx.auth.get(&color_turn).map_or(false, |valid_addr| { addr == *valid_addr })
}

#[rpc_method]
pub async fn password(addr: SocketAddr, ctx: Arc<Mutex<Context>>, params: Params) -> Response {
  let mut ctx = ctx.lock().await;
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
        ctx.auth.insert(color, addr);
        Response::success(serde_json::to_value::<ChessResponse>(chess_response).unwrap(), None)
      } else {
        let error = JsonRpcError { code: 1, message: "Incorrect password".to_string(), data: None };
        Response::error(error, None)
      }
    },
    _ => {
      let error = JsonRpcError { code: INVALID_PARAMS, message: "Invalid Params".to_string(), data: None };
      Response::error(error, None)
    }
  }
}

#[rpc_method]
pub async fn movement(_addr: SocketAddr, ctx: Arc<Mutex<Context>>, params: Params) -> Response {
  let mut ctx = ctx.lock().await;
  match params.first() {
    Some(Value::String(movement)) => {
      ctx.game.move_piece(movement.trim().to_string()).map(|_| {
        let playing_color = Game::static_playing_color(ctx.game.turn);

        let chess_response = ChessResponse {
          player_color: None,
          turn: ctx.game.turn,
          board: ctx.game.print_board(!playing_color),
          game_state: ctx.game.state,
        };

        ctx.playing_color_tx.send(playing_color).unwrap();

        Response::success(serde_json::to_value::<ChessResponse>(chess_response).unwrap(), None)
      }).unwrap_or_else(|err| {
        let error = JsonRpcError { code: INVALID_PARAMS, message: format!("{}", err), data: None };
        Response::error(error, None)
      })
    },
    _ => {
      let error = JsonRpcError { code: INVALID_PARAMS, message: "Invalid Params".to_string(), data: None };
      Response::error(error, None)
    }
  }
}


#[rpc_method]
pub async fn notify_turn(_addr: SocketAddr, ctx: Arc<Mutex<Context>>, params: Params) -> Response {
  let mut rx: Receiver<Color>;

  if let Some(playing_color_value) = params.first() {
    // Mutex in its own scope to drop it after subscribing to the channel;
    // We want to avoid blocking the Mutex for other rpc calls
    // (`movement` method should be able to lock the Mutex and write into the channel)
    {   
      let ctx = ctx.lock().await;
      rx = ctx.playing_color_tx.subscribe();
    }

    let playing_color = serde_json::from_value::<Color>(playing_color_value.clone()).unwrap(); // TODO map_err and send Response::error
    
    // Waiting for `movement` to write into the channel to continue
    while rx.recv().await.unwrap() != playing_color {
      sleep(Duration::from_secs(1)).await;
    }

    // Only when recived the expected turn color, we procced to lock the Mutex again
    let ctx = ctx.lock().await;

    let chess_response = ChessResponse {
      player_color: None,
      turn: ctx.game.turn,
      board: ctx.game.print_board(playing_color),
      game_state: ctx.game.state,
    };
    Response::success(serde_json::to_value::<ChessResponse>(chess_response).unwrap(), None)
  } else {
    let error = JsonRpcError { code: INVALID_PARAMS, message: "Invalid Params".to_string(), data: None };
    Response::error(error, None)
  }
}

pub(super) fn rpc(ctx: Context) -> Arc<Rpc<'static>> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password, false);
  rpc.register_method("movement".to_string(), movement, true);
  rpc.register_method("notify_turn".to_string(), notify_turn, false);
  Arc::new(rpc)
}
