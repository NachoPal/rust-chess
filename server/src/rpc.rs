use json_rpc::{Rpc, Response, Params};
use serde_json::Value;
use std::collections::HashMap;
use chess_lib::game::{Game, GameState, Player};
use chess_lib::pieces::{Piece, Color::{self, Black, White}};
use json_rpc_proc_macros::rpc;

#[rpc]
pub struct ChessContext<'a> {
  pub passwords: &'a HashMap<String, Color>,
  pub game: &'a Game<'a>,
}

trait ContextHandler {

  type Hola: Send;
  fn hello() {
  
  }
}

// impl <'a>ContextHandler for ChessContext<'a> {
//   type Context = Self;
// }

fn password<Context>(ctx: &Context, params: Params) -> Response {
  // ctx.game;
  // let password
  // match params[0] {
  //   Value:: String(received_password) if received_password == password => 
  // }
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
