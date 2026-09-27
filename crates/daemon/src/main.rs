mod bridge;
mod grpc_client;

use grpc_client::GrpcClient;
use tracing::{debug, info};

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("installing Ctrl+C handler");

    debug!("Shutdown signal received.");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    info!("Bride daemon started.");

    let mut client = GrpcClient::connect("http://127.0.0.1:50051")
        .await
        .expect("connected grpc client");

    let result = client
        .multiplication(10.0, 5.0)
        .await
        .expect("valid request");

    println!("Result {result}");

    let bridge = bridge::UnixSocket::new("/tmp/math.sock")
        .await
        .expect("working unix socket");

    bridge.run(shutdown_signal()).await;

    info!("Bride daemon shutdown.")
}
