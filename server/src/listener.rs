use tokio::net::TcpListener;

pub async fn tcp_listener() -> Result<TcpListener, std::io::Error> {
    // Bind the listener to the address
    TcpListener::bind("127.0.0.1:8080").await
}
