mod grpc_client;

use grpc_client::GrpcClient;
use tracing::info;

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

    info!("Bride daemon shutdown.")
}
