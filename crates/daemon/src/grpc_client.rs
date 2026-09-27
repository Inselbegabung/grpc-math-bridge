use server::server::proto::math::{OperationRequest, math_service_client::MathServiceClient};
use tonic::transport::{Channel, Error};

#[derive(Debug, Clone)]
pub struct GrpcClient {
    client: MathServiceClient<Channel>,
}

#[allow(unused)]
impl GrpcClient {
    pub async fn connect(endpoint: impl Into<String>) -> Result<Self, Error> {
        let client = MathServiceClient::connect(endpoint.into()).await?;

        Ok(Self { client })
    }

    pub async fn addition(&mut self, lhs: f64, rhs: f64) -> Result<f64, tonic::Status> {
        let response = self.client.addition(OperationRequest { lhs, rhs }).await?;

        Ok(response.into_inner().result)
    }

    pub async fn subtraction(&mut self, lhs: f64, rhs: f64) -> Result<f64, tonic::Status> {
        let response = self
            .client
            .subtraction(OperationRequest { lhs, rhs })
            .await?;

        Ok(response.into_inner().result)
    }

    pub async fn multiplication(&mut self, lhs: f64, rhs: f64) -> Result<f64, tonic::Status> {
        let response = self
            .client
            .multiplication(OperationRequest { lhs, rhs })
            .await?;

        Ok(response.into_inner().result)
    }

    pub async fn division(&mut self, lhs: f64, rhs: f64) -> Result<f64, tonic::Status> {
        let response = self.client.division(OperationRequest { lhs, rhs }).await?;

        Ok(response.into_inner().result)
    }
}
