mod grpc_client;

use grpc_client::GrpcClient;

#[tokio::main]
async fn main() {
    let mut client = GrpcClient::connect("http://127.0.0.1:50051")
        .await
        .expect("connected grpc client");

    let result = client
        .multiplication(10.0, 5.0)
        .await
        .expect("valid request");

    println!("Result {result}")
}
