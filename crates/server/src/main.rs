use clap::Parser;
use server_lib::server;
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(about = "A simple gRPC server which provides basic math functions.")]
struct Cli {
    /// Host address to bind the gRPC server to.
    #[arg(long, default_value = "127.0.0.1", env = "MATH_GRPC_HOST")]
    host: Ipv4Addr,

    /// Port to bind the gRPC server to.
    #[arg(short, long, default_value_t = 50051, env = "MATH_GRPC_PORT")]
    port: u16,
}

impl Cli {
    pub fn host_socket(&self) -> SocketAddr {
        self.host_socket_str().parse().expect("valid socket")
    }

    pub fn host_socket_str(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let addr = cli.host_socket();

    server::run(addr).await;
}
