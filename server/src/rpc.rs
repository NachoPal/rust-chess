use json_rpc::{JsonRpcError, Params, Response, Id, INVALID_PARAMS};
use serde_json::{json, Value};
use super::{Rpc, Context};
use std::sync::Arc;

fn password(ctx: Arc<Context>, params: Params) -> Response {
  let expected_paswords = &ctx.passwords;
  match params.first() {
    Some(Value::String(pasword)) => {
      if let Some(color) = ctx.passwords.get(pasword) {
        let result = json!(ctx.game.print_board());
        Response::success(result, None)
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
