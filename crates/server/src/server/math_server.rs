use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("math");
}

pub use proto::math_service_server::MathServiceServer;

#[derive(Debug, Default)]
pub struct MathServer;

#[tonic::async_trait]
impl proto::math_service_server::MathService for MathServer {
    async fn addition(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let request = request.into_inner();

        Ok(Response::new(proto::OperationResult {
            result: request.lhs + request.rhs,
        }))
    }

    async fn subtraction(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let request = request.into_inner();

        Ok(Response::new(proto::OperationResult {
            result: request.lhs - request.rhs,
        }))
    }

    async fn multiplication(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let request = request.into_inner();

        Ok(Response::new(proto::OperationResult {
            result: request.lhs * request.rhs,
        }))
    }

    async fn division(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let request = request.into_inner();

        if request.rhs == 0.0 {
            return Err(Status::invalid_argument("division by zero is not allowed"));
        }

        Ok(Response::new(proto::OperationResult {
            result: request.lhs / request.rhs,
        }))
    }
}
