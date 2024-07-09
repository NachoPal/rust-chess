use json_rpc::{Response, Params};
use serde_json::Value;
use super::{Rpc, Context};

fn password<Context>(ctx: &Context, params: Params) -> Response {
  Response::Success {
    jsonrpc: "2.0".to_string(),
    result: Value::Null,

  }
}

pub(super) fn rpc<'a>(ctx: &'a Context<'a>) -> Rpc<'a> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password);
  rpc
}
