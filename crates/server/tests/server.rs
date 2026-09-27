use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use server_lib::server::{
    proto::math::{OperationRequest, math_service_client::MathServiceClient},
    run,
};
use tokio::task::JoinHandle;

async fn start_server(socket_addr: SocketAddr) -> JoinHandle<()> {
    let server = tokio::spawn(async move {
        run(socket_addr).await;
    });

    // Give the server time to start, optional we can rework the `run` function to accept a `TcpListener`
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    server
}

#[tokio::test]
async fn test_addition() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);
    let server = start_server(socket_addr).await;

    let mut client = MathServiceClient::connect(format!("http://{socket_addr}"))
        .await
        .expect("connect to gRPC server");

    let response = client
        .addition(OperationRequest {
            lhs: 10.0,
            rhs: 5.0,
        })
        .await
        .expect("addition request")
        .into_inner();

    assert_eq!(response.result, 15.0);

    server.abort();
}

#[tokio::test]
async fn test_subtraction() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);
    let server = start_server(socket_addr).await;

    let mut client = MathServiceClient::connect(format!("http://{socket_addr}"))
        .await
        .expect("connect to gRPC server");

    let response = client
        .subtraction(OperationRequest {
            lhs: 10.0,
            rhs: 5.0,
        })
        .await
        .expect("addition request")
        .into_inner();

    assert_eq!(response.result, 5.0);

    server.abort();
}

#[tokio::test]
async fn test_multiplication() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);
    let server = start_server(socket_addr).await;

    let mut client = MathServiceClient::connect(format!("http://{socket_addr}"))
        .await
        .expect("connect to gRPC server");

    let response = client
        .multiplication(OperationRequest {
            lhs: 10.0,
            rhs: 5.0,
        })
        .await
        .expect("addition request")
        .into_inner();

    assert_eq!(response.result, 50.0);

    server.abort();
}

#[tokio::test]
async fn test_division() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);
    let server = start_server(socket_addr).await;

    let mut client = MathServiceClient::connect(format!("http://{socket_addr}"))
        .await
        .expect("connect to gRPC server");

    let response = client
        .division(OperationRequest {
            lhs: 10.0,
            rhs: 5.0,
        })
        .await
        .expect("division request")
        .into_inner();

    assert_eq!(response.result, 2.0);

    let response = client
        .division(OperationRequest {
            lhs: 10.0,
            rhs: 0.0,
        })
        .await;
    assert!(response.is_err());

    server.abort();
}

#[tokio::test]
async fn test_errors() {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 50052);
    let server = start_server(socket_addr).await;

    let mut client = MathServiceClient::connect(format!("http://{socket_addr}"))
        .await
        .expect("connect to gRPC server");

    let response = client
        .multiplication(OperationRequest {
            lhs: f64::NAN,
            rhs: 1.0,
        })
        .await;
    assert!(response.is_err());

    let response = client
        .multiplication(OperationRequest {
            lhs: f64::INFINITY,
            rhs: 1.0,
        })
        .await;
    assert!(response.is_err());

    let response = client
        .multiplication(OperationRequest {
            lhs: f64::MAX,
            rhs: 2.0,
        })
        .await;
    assert!(response.is_err());

    server.abort();
}
