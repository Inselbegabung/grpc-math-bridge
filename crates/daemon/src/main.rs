mod bridge;
mod config;
mod grpc_client;
mod handler;

use clap::Parser;
use config::Config;
use grpc_client::GrpcClient;
use std::path::PathBuf;
use tracing::{debug, info};

#[derive(Debug, Parser)]
#[command(version, about = "gRPC math bridge daemon using a Unix socket")]
struct Args {
    /// Path to the daemon configuration file
    config: PathBuf,
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("installing Ctrl+C handler");

    debug!("Shutdown signal received.");
}

fn unwrap_or_exit<T, E: std::fmt::Display>(result: Result<T, E>, context: &str) -> T {
    result.unwrap_or_else(|err| {
        eprintln!("{context}: {err}");
        std::process::exit(1);
    })
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    let config = unwrap_or_exit(Config::load(&args.config).await, "Failed to load config");

    info!("Bridge daemon started.");

    let client = unwrap_or_exit(
        GrpcClient::connect(config.grpc_address).await,
        "Failed to connect to gRPC server",
    );

    let handler = handler::Handler::new(client);
    let bridge = unwrap_or_exit(
        bridge::UnixSocket::new(&config.socket_path).await,
        "Failed to initialize Unix socket",
    );

    bridge.run(handler, shutdown_signal()).await;

    info!("Bridge daemon shutdown.")
}
