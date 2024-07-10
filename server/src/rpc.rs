use json_rpc::{JsonRpcError, Params, Response};
use serde_json::{json, Value};
use super::{Rpc, Context};
use std::sync::Arc;

fn password(id: u32, ctx: Arc<Context>, params: Params) -> Response {
  let expected_paswords = &ctx.passwords;
  match params.first() {
    Some(Value::String(pasword)) => {
      if let Some(color) = ctx.passwords.get(pasword) {
        Response::Success {
          jsonrpc: "2.0".to_string(),
          result: json!(ctx.game.print_board()),
        }
      } else {
        Response::Error {
          jsonrpc: "2.0".to_string(),
          error: JsonRpcError { code: -1, message: "Incorrect password".to_string(), data: None },
          id
        }
      }
    },
    _ => {
      Response::Error {
        jsonrpc: "2.0".to_string(),
        error: JsonRpcError { code: -1, message: "Incorrect password".to_string(), data: None },
        id
      }
    }
  }

}

pub(super) fn rpc(ctx: Arc<Context>) -> Arc<Rpc> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password);
  Arc::new(rpc)
}
