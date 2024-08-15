//! Rust Chess Client
//!
//! Client that connects with Rust Chess Server
//! asking for movements and printing the board
use std::sync::Arc;
use tokio::{
    net::TcpStream,
    signal,
    sync::{mpsc, Mutex},
};

mod rpc;
mod run;
mod socket;

use rpc::{movement, password};
use run::run;

/// Connect to the Chess Server and guide (ask for movement or wait for opponent's movement)
/// throughthe whole game lifetime
#[tokio::main]
async fn main() -> core::result::Result<(), String> {
    // Connect to the server
    let socket = TcpStream::connect("127.0.0.1:8080").await.map_err(|err| { format!("{:?}", err)})?;

    let (reader, mut writer) = tokio::io::split(socket);
    let reader_mutex = Arc::new(Mutex::new(reader));

    // Handle closing socket connection
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(32);
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        if let Ok(_) = signal::ctrl_c().await {
            println!("\n\nPress ENTER to exit");
            let _ = shutdown_tx_clone.send("Received Ctrl+C, shutting down.").await;
        }
    });

    let result = tokio::select! {
        result = run(reader_mutex.clone(), &mut writer) => { result },
        message = shutdown_rx.recv() => {
            eprintln!("{:?}", message.expect("msg exists"));
            Ok(message.expect("msg exists").to_string())
        }
    };

    result.map(|msg| {
        println!("{:?}", msg);
        ()
    }).map_err(|_| "Connection closed by Server".to_string() )
}
