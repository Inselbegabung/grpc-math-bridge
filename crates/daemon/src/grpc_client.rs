use crate::handler::MathService;
use server::server::proto::math::{OperationRequest, math_service_client::MathServiceClient};
use tonic::transport::{Channel, Error};

#[derive(Debug, Clone)]
pub struct GrpcClient {
    math: MathServiceClient<Channel>,
}

#[allow(unused)]
impl GrpcClient {
    pub async fn connect(endpoint: impl Into<String>) -> Result<Self, Error> {
        let math = MathServiceClient::connect(endpoint.into()).await?;

        Ok(Self { math })
    }
}

#[async_trait::async_trait]
impl MathService for GrpcClient {
    async fn addition(&mut self, lhs: f64, rhs: f64) -> Result<f64, String> {
        let response = self
            .math
            .addition(OperationRequest { lhs, rhs })
            .await
            .map_err(|e| e.to_string())?;

        Ok(response.into_inner().result)
    }

    async fn subtraction(&mut self, lhs: f64, rhs: f64) -> Result<f64, String> {
        let response = self
            .math
            .subtraction(OperationRequest { lhs, rhs })
            .await
            .map_err(|e| e.to_string())?;

        Ok(response.into_inner().result)
    }

    async fn multiplication(&mut self, lhs: f64, rhs: f64) -> Result<f64, String> {
        let response = self
            .math
            .multiplication(OperationRequest { lhs, rhs })
            .await
            .map_err(|e| e.to_string())?;

        Ok(response.into_inner().result)
    }
    async fn division(&mut self, lhs: f64, rhs: f64) -> Result<f64, String> {
        let response = self
            .math
            .division(OperationRequest { lhs, rhs })
            .await
            .map_err(|e| e.to_string())?;

        Ok(response.into_inner().result)
    }
}
