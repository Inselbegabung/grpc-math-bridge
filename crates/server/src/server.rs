mod math_server;

use math_server::{MathServer, MathServiceServer};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tracing::{debug, info};

pub mod proto {
    use super::math_server;
    pub use math_server::proto as math;
}

pub async fn run(socket_addr: SocketAddr) {
    let listener = TcpListener::bind(socket_addr).await.expect("bind server");

    run_with_listener(listener).await;
}

pub async fn run_with_listener(listener: TcpListener) {
    let socket_addr = listener.local_addr().expect("get local server address");
    info!("Start server on address '{socket_addr}'");

    let math_service = MathServer;
    let incoming = TcpListenerStream::new(listener);

    Server::builder()
        .add_service(MathServiceServer::new(math_service))
        .serve_with_incoming_shutdown(incoming, shutdown_signal())
        .await
        .expect("running gRPC server");
    info!("Start stopped.");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("installing Ctrl+C handler");

    debug!("Shutdown signal received.");
}
