use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use colored::*;
// pub use derive_proc_macros::rpc;

pub const INVALID_PARAMS: i32 = -32602;
pub const METHOD_NOT_FOUND: i32 = -32601;

pub type Params = Vec<Value>;
pub type Id = u32;

#[derive(Serialize, Deserialize, Debug)]
pub struct Version(String);

impl Default for Version {
  fn default() -> Self {
      Self("2.0".to_string())
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  pub jsonrpc: Version,
  pub method: String,
  pub params: Params,
  pub id: Id,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Response {
  Success {
    jsonrpc: Version,
    result: Value,
    id: Id,
  },
  Error {
    jsonrpc: Version,
    error: JsonRpcError,
    id: Id,
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcError {
  pub code: i32,
  pub message: String,
  pub data: Option<Value>
}

impl Request {
  pub fn new(method: String, params: Params, id: Option<Id>) -> Self {
    Request { jsonrpc: Version::default(), method, params, id: id.unwrap_or_default() }
  }

  pub fn set_id(&mut self, new_id: Id) {
    self.id = new_id;
  }
}

impl Response {
  pub fn is_success(&self) -> bool {
      matches!(self, Response::Success { .. })
  }

  pub fn is_error(&self) -> bool {
    matches!(self, Response::Error { .. })
}

  pub fn result(&self) -> Result<(&Value, Id), (&JsonRpcError, Id)> {
    if let Response::Success { result, id, .. } = self {
      Ok((result, *id))
    } else if let Response::Error { error, id, .. } = self {
      Err((error, *id))
    } else {
      unreachable!()
    }
  }

  pub fn set_id(&mut self, new_id: Id) {
    match self {
      Response::Success { id, .. } => *id = new_id,
      Response::Error { id, .. } => *id = new_id,
    }
  }

  pub fn success(result: Value, id: Option<Id>) -> Response {
    Response::Success {
      jsonrpc: Version::default(),
      result,
      id: id.unwrap_or_default(),
    }
  }

  pub fn error(error: JsonRpcError, id: Option<Id>) -> Response {
    Response::Error {
      jsonrpc: Version::default(),
      error,
      id: id.unwrap_or_default()
    }
  }
}

impl Display for Response {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self.result() {
      Ok((result, _)) => {
        write!(f, "{}", result)
      },
      Err((err, _)) => {
        let error_msg = format!("Error");
        write!(f, "{}: {}", error_msg.red(), err.message)
      }
    }
  }
}
