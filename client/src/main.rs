use std::{io, sync::Arc};
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

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server
    let socket = TcpStream::connect("127.0.0.1:8080").await?;

    let (reader, mut writer) = tokio::io::split(socket);
    let reader_mutex = Arc::new(Mutex::new(reader));

    // Handle closing socket connection
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(32);
    let shutdown_tx_clone = shutdown_tx.clone();
    tokio::spawn(async move {
        if let Ok(_) = signal::ctrl_c().await {
            let _ = shutdown_tx_clone.send("Received Ctrl+C, shutting down.");
        }
    });

    tokio::select! {
      _ = run(reader_mutex.clone(), &mut writer, shutdown_tx.clone()) => {},
      message = shutdown_rx.recv() => {
        eprintln!("{:?}", message);
      }
    }
    Ok(())
}
