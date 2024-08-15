//! Run module.
//!
//! Collection of Rpc `Request` to be submitted to the server
//!
use core::net::SocketAddr;
use std::{io, sync::Arc};
use tokio::{
    io::{ReadHalf, WriteHalf},
    net::TcpStream,
    sync::{broadcast::Receiver, Mutex},
};

use super::{
    rpc::{notify_close_connection, notify_turn, Rpc},
    socket,
};

async fn respond(
    reader: &mut ReadHalf<TcpStream>,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
    addr: SocketAddr,
    rpc: Arc<Rpc<'static>>,
) -> io::Result<()> {
    let request = socket::read(reader).await?;
    let id = request.id;
    let name = request.method;
    let params = request.params;
    let response = rpc.call_method(addr, id, name.clone(), params).await;

    if name == "password" && response.is_success() {
        tokio::task::spawn(notify_turn(
            rpc.clone(),
            Arc::clone(&writer),
            response.clone(),
        ));
    }

    socket::write(Arc::clone(&writer), response).await
}

pub(super) async fn run(
    mut reader: ReadHalf<TcpStream>,
    writer: Arc<Mutex<WriteHalf<TcpStream>>>,
    addr: SocketAddr,
    rpc: Arc<Rpc<'static>>,
) {
    // Spawn a new task to handle the connection
    let mut new_addr_channel_rx: Receiver<SocketAddr>;
    {
        let ctx = rpc.ctx.lock().await;
        new_addr_channel_rx = ctx.auth.new_addr_channel_tx.subscribe();
    }

    loop {
        let writer: Arc<Mutex<WriteHalf<TcpStream>>> = Arc::clone(&writer);

        tokio::select! {
            response = respond(&mut reader, writer.clone(), addr, rpc.clone()) => {
                if let Err(e) = response {
                    eprintln!("{:?}", e);
                    return;
                }
            },
            addr_to_close = new_addr_channel_rx.recv() => {
                if addr_to_close == Ok(addr) {
                eprintln!("Another connection has been established. Dropping {:?}", addr);
                notify_close_connection(writer.clone()).await;
                break
                }
            }
        }
    }
}
