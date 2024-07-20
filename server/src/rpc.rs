use json_rpc::{JsonRpcError, Params, Response, INVALID_PARAMS};
use serde_json::Value;
use super::{Rpc, Context};
use std::sync::{Arc, Mutex};
use chess_server::ChessResponse;
use chess_lib::pieces::Color;
use json_rpc_proc_macros::rpc_method;

#[rpc_method]
pub async fn password(ctx: Arc<Mutex<Context>>, params: Params) -> Response {
  let ctx = ctx.lock().unwrap();
  match params.first() {
    Some(Value::String(pasword)) => {
      if let Some(color) = ctx.passwords.get(pasword) {
        let chess_response = ChessResponse {
          color: *color,
          turn: ctx.game.turn,
          board: ctx.game.print_board(),
        };
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
pub async fn movement(ctx: Arc<Mutex<Context>>, params: Params) -> Response {
  let mut ctx = ctx.lock().unwrap();
  match params.first() {
    Some(Value::String(movement)) => {
      ctx.game.move_piece(movement.trim().to_string()).map(|_| {
      let chess_response = ChessResponse {
        color: Color::White, /* TODO: Probably need to remove it */
        turn: ctx.game.turn,
        board: ctx.game.print_board(),
      };
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

pub(super) fn rpc(ctx: Context) -> Arc<Rpc<'static>> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password);
  rpc.register_method("movement".to_string(), movement);
  Arc::new(rpc)
}
