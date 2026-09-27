use tracing::info;

#[async_trait::async_trait]
pub trait Handle {
    async fn handle(&mut self, input: Vec<u8>) -> Vec<u8>;
}

#[async_trait::async_trait]
#[allow(unused)]
pub trait MathService {
    async fn addition(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
    async fn subtraction(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
    async fn multiplication(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
    async fn division(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
}

#[derive(Clone)]
pub struct Handler<C> {
    #[allow(unused)]
    grpc_client: C,
}

impl<C> Handler<C> {
    pub fn new(grpc_client: C) -> Self {
        Self { grpc_client }
    }
}

#[async_trait::async_trait]
impl<C> Handle for Handler<C>
where
    C: MathService + Send + Sync,
{
    async fn handle(&mut self, input: Vec<u8>) -> Vec<u8> {
        info!("Message received,{input:?}");

        // todo: parse message
        // todo create gRPC request
        // todo: parse response

        b"Not implemented yet".to_vec()
    }
}
