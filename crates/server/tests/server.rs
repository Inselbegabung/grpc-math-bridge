use std::net::{Ipv4Addr, SocketAddr};

use server::server::{
    proto::math::{OperationRequest, math_service_client::MathServiceClient},
    run_with_listener,
};
use tokio::{net::TcpListener, task::JoinHandle};

async fn start_server() -> (SocketAddr, JoinHandle<()>) {
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
        .await
        .expect("bind test server");

    let socket_addr = listener.local_addr().expect("get test server address");

    let server = tokio::spawn(async move {
        run_with_listener(listener).await;
    });

    (socket_addr, server)
}

#[tokio::test]
async fn test_addition() {
    let (socket_addr, server) = start_server().await;

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
    let (socket_addr, server) = start_server().await;

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
    let (socket_addr, server) = start_server().await;

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
    let (socket_addr, server) = start_server().await;

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
    let (socket_addr, server) = start_server().await;

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
