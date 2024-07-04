use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  jsonrpc: String,
  method: String,
  params: Value,
  id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSuccess {
  jsonrpc: String,
  result: Value,
  id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseError {
  jsonrpc: String,
  error: JsonRpcError,
  id: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcError {
  code: i32,
  message: String,
  data: Option<Value>
}
