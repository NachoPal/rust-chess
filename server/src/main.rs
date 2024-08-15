//! Rust Chess Server
//!
//! Server that initializes a Chess Game
//!
//! Waits for two clients (White & Black) to connect
use chess_lib::board::{Board, Position};
use chess_lib::game::Game;
use chess_lib::pieces::{Color, Piece};
use clap::Parser;
use std::{collections::HashMap, io, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::{broadcast, Mutex},
};

mod rpc;
mod run;
mod socket;

use rpc::{rpc, Authentication, Context};
use run::run;

const MAX_CHANNEL: usize = 16;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// White Password
    #[arg(short, long)]
    white: String,

    /// Black Password
    #[arg(short, long)]
    black: String,

    /// Server address
    #[arg(short, long)]
    address: String,

    /// Server port
    #[arg(short, long)]
    port: String,
}

pub fn clean_terminal() {
    print!("\x1B[2J\x1B[H");
}

/// Initialize a Chess game and runs the main loop to keep
/// listening new Tcp connections
#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();
    let dimension = Position { x: 7, y: 7 };
    let pieces: Option<Vec<(Position, Box<dyn Piece>)>> = None;
    let board = Board::new(dimension, pieces);
    let mut game = Game::new(board);

    game.set_board();
    game.start();

    let mut passwords = HashMap::new();
    passwords.insert(args.white, Color::White);
    passwords.insert(args.black, Color::Black);

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
    let address = args.address + ":" + &args.port;
    let listener = TcpListener::bind(address).await?;

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
