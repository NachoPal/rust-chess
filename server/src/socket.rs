//! Socket module.
//!
//! Methods to communicate (read & write) with the Chess Client
//!
use futures::Future;
use json_rpc::{Request, Response};
use std::{
    io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf},
    net::TcpStream,
    sync::Mutex,
};

struct ReadSocket {
    lock: Arc<Mutex<ReadHalf<TcpStream>>>,
}

impl Future for ReadSocket {
    type Output = Vec<u8>;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        // lock the socket
        match unsafe { Pin::new_unchecked(&mut self.lock.lock()).poll(ctx) } {
            // If lock is available I try to read from the socket
            Poll::Ready(mut socket_guard) => {
                let mut buf = vec![0; 90000];
                let mut read_future = socket_guard.read(&mut buf);

                match unsafe { Pin::new_unchecked(&mut read_future).poll(ctx) } {
                    // If data is ready to be read in socket I return
                    Poll::Ready(n) => {
                        return Poll::Ready(
                            buf[..n.expect("Connection closed by Server")].to_vec(),
                        );
                    }
                    Poll::Pending => {
                        drop(socket_guard);
                        ctx.waker().wake_by_ref();
                        return Poll::Pending;
                    }
                }
            }
            Poll::Pending => {
                ctx.waker().wake_by_ref();
                return Poll::Pending;
            }
        }
    }
}

pub async fn read(reader: &mut ReadHalf<TcpStream>) -> io::Result<Request> {
    let mut buf = [0; 90000];
    let n = reader.read(&mut buf).await?;

    if n != 0 {
        let request = std::str::from_utf8(&buf[0..n]).unwrap();
        return Ok(serde_json::from_str::<Request>(request).unwrap());
    } else {
        return Err(std::io::ErrorKind::ConnectionReset.into());
    }
}

pub async fn write(
    writer_mutex: Arc<Mutex<WriteHalf<TcpStream>>>,
    response: Response,
) -> io::Result<()> {
    let response = serde_json::to_string::<Response>(&response).unwrap();

    let mut writer = writer_mutex.lock().await;
    writer.write_all(&response.as_bytes()).await
}
