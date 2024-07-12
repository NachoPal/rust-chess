use json_rpc::{JsonRpcError, Params, Response, INVALID_PARAMS};
use serde_json::Value;
use super::{Rpc, Context};
use std::sync::Arc;
use chess_server::ChessResponse;
use chess_lib::pieces::Color;
fn password(ctx: Arc<Context>, params: Params) -> Response {
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

fn movement(ctx: Arc<Context>, params: Params) -> Response {
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

pub(super) fn rpc(ctx: Arc<Context>) -> Arc<Rpc> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password);
  rpc.register_method("movement".to_string(), movement);
  Arc::new(rpc)
}
