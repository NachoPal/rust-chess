use chess_lib::board::{Board, Position};
use chess_lib::game::Game;
use chess_lib::pieces::{Color, Piece};
use std::{collections::HashMap, io, sync::Arc};
use tokio::sync::{broadcast, Mutex};

mod listener;
mod rpc;
mod run;
mod socket;

use listener::tcp_listener;
use rpc::{rpc, Authentication, Context};
use run::run;

const MAX_CHANNEL: usize = 16;

pub fn clean_terminal() {
    print!("{esc}c", esc = 27 as char);
}

#[tokio::main]
async fn main() -> io::Result<()> {
    clean_terminal();
    let dimension = Position { x: 7, y: 7 };

    let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
    let board = Board::new(dimension, pieces);
    let mut game = Game::new(board);

    game.set_board();
    game.start();

    let mut passwords = HashMap::new();
    passwords.insert("white".to_string(), Color::White);
    passwords.insert("black".to_string(), Color::Black);

    let (new_addr_channel_tx, _) = broadcast::channel(MAX_CHANNEL);
    let auth = Authentication {
        addrs: HashMap::new(),
        new_addr_channel_tx,
    };

    let (playing_color_tx, _) = broadcast::channel(MAX_CHANNEL);

    let ctx = Context {
        passwords,
        auth,
        game,
        playing_color_tx,
    };

    let rpc = rpc(ctx);
    let listener = tcp_listener().await?;

    println!("Waiting for connections...\n");

    loop {
        // Accept a new socket
        let (socket, addr) = listener.accept().await?;
        let (reader, writer) = tokio::io::split(socket);
        let writer_mutex = Arc::new(Mutex::new(writer));
        println!("- Established connection with {:?}", addr);
        tokio::spawn(run(reader, writer_mutex, addr, rpc.clone()));
    }
}
