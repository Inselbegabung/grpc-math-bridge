use tracing::{debug, warn};

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait Handle {
    async fn handle(&mut self, input: Vec<u8>) -> Vec<u8>;
}

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait MathService {
    async fn addition(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
    async fn subtraction(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
    async fn multiplication(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
    async fn division(&mut self, lhs: f64, rhs: f64) -> Result<f64, String>;
}

#[derive(Clone)]
pub struct Handler<M> {
    grpc_client: M,
}

impl<M> Handler<M> {
    pub fn new(grpc_client: M) -> Self {
        Self { grpc_client }
    }
}

#[async_trait::async_trait]
impl<M> Handle for Handler<M>
where
    M: MathService + Send + Sync,
{
    async fn handle(&mut self, input: Vec<u8>) -> Vec<u8> {
        debug!("Message received,{input:?}");

        match handle(&mut self.grpc_client, input).await {
            Ok(response) => response,
            Err(err) => {
                protocol::create_response(protocol::MathResult::error(err)).unwrap_or_else(|e| {
                    warn!("The protocol parser failed to create a response '{e:?}'");
                    b"Internal Error".to_vec()
                })
            }
        }
    }
}

async fn handle(math: &mut impl MathService, input: Vec<u8>) -> Result<Vec<u8>, String> {
    let request =
        protocol::parse_request(&input).map_err(|e| format!("Parse request failed: {e:?}"))?;
    let result = handle_command(math, request).await?;
    protocol::create_response(result).map_err(|e| {
        warn!("Parse response failed: {e:?}");
        "Internal error".to_string()
    })
}

async fn handle_command(
    math: &mut impl MathService,
    command: protocol::Command,
) -> Result<protocol::MathResult, String> {
    match command {
        protocol::Command::Addition(data) => {
            let result = math.addition(data.lhs, data.rhs).await?;
            Ok(protocol::MathResult::result(result))
        }
        protocol::Command::Subtraction(data) => {
            let result = math.subtraction(data.lhs, data.rhs).await?;
            Ok(protocol::MathResult::result(result))
        }
        protocol::Command::Multiplication(data) => {
            let result = math.multiplication(data.lhs, data.rhs).await?;
            Ok(protocol::MathResult::result(result))
        }
        protocol::Command::Division(data) => {
            let result = math.division(data.lhs, data.rhs).await?;
            Ok(protocol::MathResult::result(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handle_addition() {
        let mut math_service = MockMathService::new();

        math_service
            .expect_addition()
            .with(mockall::predicate::eq(10.0), mockall::predicate::eq(5.0))
            .times(1)
            .returning(|_, _| Ok(15.0));

        let mut handler = Handler::new(math_service);

        let request =
            br#"{"version":"1.0","command":"ADDITION","data":{"lhs":10.0,"rhs":5.0}}"#.to_vec();

        let response = handler.handle(request).await;

        let response = String::from_utf8(response).expect("response should be valid UTF-8");

        assert_eq!(response, r#"{"type":"RESULT","data":{"result":15.0}}"#);
    }
}
