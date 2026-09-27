mod math_server;

use math_server::{MathServer, MathServiceServer};
use std::net::SocketAddr;
use tonic::transport::Server;
use tracing::{debug, info};

pub mod proto {
    use super::math_server;
    pub use math_server::proto as math;
}

pub async fn run(socket_addr: SocketAddr) {
    let math_service = MathServer;

    info!("Start server on address '{socket_addr:?}'");
    Server::builder()
        .add_service(MathServiceServer::new(math_service))
        .serve_with_shutdown(socket_addr, shutdown_signal())
        .await
        .expect("running gRPC server");
    info!("Start shutdown.");
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("installing Ctrl+C handler");

    debug!("Shutdown signal received.");
}
