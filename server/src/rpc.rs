use json_rpc::{Rpc, Response, Params};
use serde_json::Value;

fn password<Context>(ctx: &Context, params: Params) -> Response {
  Response::Success {
    jsonrpc: "2.0".to_string(),
    result: Value::Null,

  }
}

pub(super) fn rpc<Context: Send + std::marker::Sync>(ctx: &Context) -> Rpc<Context> {
  let mut rpc = Rpc::new(ctx);
  rpc.register_method("password".to_string(), password);
  rpc
}
