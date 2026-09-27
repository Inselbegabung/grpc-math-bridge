use tonic::{Request, Response, Status};

pub mod proto {
    tonic::include_proto!("math");
}

pub use proto::math_service_server::MathServiceServer;

#[derive(Debug, Default)]
pub struct MathServer;

struct OperationData {
    lhs: f64,
    rhs: f64,
}

impl TryFrom<proto::OperationRequest> for OperationData {
    type Error = Status;
    fn try_from(value: proto::OperationRequest) -> Result<Self, Self::Error> {
        check_finite(value.lhs)?;
        check_finite(value.rhs)?;
        Ok(Self {
            lhs: value.lhs,
            rhs: value.rhs,
        })
    }
}

fn check_finite(value: f64) -> Result<(), Status> {
    if !value.is_finite() {
        return Err(Status::invalid_argument("Value must be finite."));
    }
    Ok(())
}

fn check_result(value: f64) -> Result<(), Status> {
    if !value.is_finite() {
        return Err(Status::out_of_range("The result is not finite."));
    }
    Ok(())
}

#[tonic::async_trait]
impl proto::math_service_server::MathService for MathServer {
    async fn addition(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let data: OperationData = request.into_inner().try_into()?;

        let result = data.lhs + data.rhs;
        check_result(result)?;

        Ok(Response::new(proto::OperationResult { result }))
    }

    async fn subtraction(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let data: OperationData = request.into_inner().try_into()?;

        let result = data.lhs - data.rhs;
        check_result(result)?;

        Ok(Response::new(proto::OperationResult { result }))
    }

    async fn multiplication(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let data: OperationData = request.into_inner().try_into()?;

        let result = data.lhs * data.rhs;
        check_result(result)?;

        Ok(Response::new(proto::OperationResult { result }))
    }

    async fn division(
        &self,
        request: Request<proto::OperationRequest>,
    ) -> Result<Response<proto::OperationResult>, Status> {
        let data: OperationData = request.into_inner().try_into()?;

        if data.rhs == 0.0 {
            return Err(Status::invalid_argument("division by zero is not allowed"));
        }

        let result = data.lhs / data.rhs;
        check_result(result)?;

        Ok(Response::new(proto::OperationResult { result }))
    }
}
