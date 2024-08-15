//! Socket module.
//!
//! Methods to communicate (read & write) with the Chess Server
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
                        return Poll::Ready(buf[..n.unwrap()].to_vec());
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

pub async fn write<R, F>(socket: &mut WriteHalf<TcpStream>, request: R) -> io::Result<()>
where
    R: FnOnce() -> F,
    F: Future<Output = Request>,
{
    let request_json = serde_json::to_string(&request().await).unwrap();
    socket.write_all(request_json.as_bytes()).await
}

pub async fn read(socket_mutex: Arc<Mutex<ReadHalf<TcpStream>>>) -> io::Result<Response> {
    let read_socket = ReadSocket { lock: socket_mutex };
    let buf = read_socket.await;
    let response = serde_json::from_slice::<Response>(&buf)?;
    Ok(response)
}

pub async fn request<R, F>(
    writer: &mut WriteHalf<TcpStream>,
    reader_mutex: Arc<Mutex<ReadHalf<TcpStream>>>,
    request: R,
) -> io::Result<Response>
where
    R: FnOnce() -> F,
    F: Future<Output = Request>,
{
    write(writer, request).await?;
    read(reader_mutex).await
}
