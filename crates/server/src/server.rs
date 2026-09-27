mod math_server;

use math_server::{MathServer, MathServiceServer};
use std::net::SocketAddr;
use tonic::transport::Server;

pub mod proto {
    use super::math_server;
    pub use math_server::proto as math;
}

pub async fn run(socket_addr: SocketAddr) {
    let math_service = MathServer::default();

    println!("Math gRPC server listening on {socket_addr:?}");

    Server::builder()
        .add_service(MathServiceServer::new(math_service))
        .serve(socket_addr)
        .await
        .expect("running gRPC server");
}
