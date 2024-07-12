use json_rpc::{JsonRpcError, Params, Response, INVALID_PARAMS};
use serde_json::Value;
use super::{Rpc, Context};
use std::sync::Arc;
use chess_server::ChessResponse;

fn password(ctx: Arc<Context>, params: Params) -> Response {
  let expected_paswords = &ctx.passwords;
  match params.first() {
    Some(Value::String(pasword)) => {
      if let Some(color) = ctx.passwords.get(pasword) {
        // let a = json!({ "color": color, "board": ctx.game.print_board() });
        // a.is
        // let result = json!({ "color": color, "board": ctx.game.print_board() });
        let chess_response = ChessResponse {
          color: *color,
          turn: ctx.game.turn,
          board: ctx.game.print_board(),
        };
        // let a = json!({ "color": color, "board": ctx.game.print_board(), "turn": ctx.game.turn });
        Response::success(serde_json::to_value::<ChessResponse>(chess_response).unwrap(), None)
        // Response::success(a, None)
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

pub(super) fn rpc(ctx: Arc<Context>) -> Arc<Rpc> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password);
  Arc::new(rpc)
}
