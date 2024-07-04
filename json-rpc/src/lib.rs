use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

pub type Params = Vec<Value>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  pub jsonrpc: String,
  pub method: String,
  pub params: Params,
  pub id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Response {
  Success {
    jsonrpc: String,
    result: Value,
  },
  Error {
    jsonrpc: String,
    error: JsonRpcError,
    id: u32,
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcError {
  pub code: i32,
  pub message: String,
  pub data: Option<Value>
}

type MethodFunction<Context> = fn(&Context, Params) -> Response;

pub struct Rpc<'a, Context> {
  pub methods: HashMap<String, MethodFunction<Context>>,
  pub ctx: &'a Context,
}

impl<'a, Context: Send + Sync> Rpc<'a, Context> {
  pub fn new(ctx: &'a Context) -> Self {
    Self {
      methods: HashMap::new(),
      ctx: ctx,
    }
  }

  pub fn register_method(&mut self, name: String, method: MethodFunction<Context>) {
    self.methods.insert(name, method);
  }

  pub fn call_method(&self, name: String, params: Params) -> Response {
    if let Some(method) = self.methods.get(&name) {
      return method(self.ctx, params)
    } else {
      Response::Error { jsonrpc: "2.0".to_string(), error: JsonRpcError { code: -1, message: "No method".to_string(), data: None }, id: 1 }
    }
  }
}
