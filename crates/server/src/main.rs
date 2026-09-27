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
        let addr_str = format!("{}:{}", self.host, self.port);
        addr_str.parse().expect("valid socket")
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let addr = cli.host_socket();
    server::run(addr).await;
}
