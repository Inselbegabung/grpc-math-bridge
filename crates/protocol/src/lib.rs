use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: &str = "1.0";

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
pub struct CommandData {
    pub lhs: f64,
    pub rhs: f64,
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq)]
#[serde(tag = "command", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Command {
    Addition(CommandData),
    Subtraction(CommandData),
    Multiplication(CommandData),
    Division(CommandData),
}

#[derive(Debug, Deserialize)]
struct Header {
    pub version: String,
}

#[derive(Debug, Deserialize)]
struct RequestBody {
    #[serde(flatten)]
    pub command: Command,
}

#[derive(Debug, Serialize)]
struct FullRequest {
    pub version: String,
    #[serde(flatten)]
    pub command: Command,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ResultData {
    pub result: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorData {
    pub error: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MathResult {
    Result(ResultData),
    Error(ErrorData),
}

#[derive(Debug, Serialize)]
pub struct MathResultResponse {
    #[serde(flatten)]
    pub result: MathResult,
}

#[derive(Debug, Serialize)]
struct FullResponse {
    version: String,

    #[serde(flatten)]
    result: MathResult,
}

#[derive(Debug)]
pub enum Error {
    Serialize(String),
    UnsupportedVersion,
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::Serialize(value.to_string())
    }
}

impl MathResult {
    pub fn result(value: f64) -> Self {
        MathResult::Result(ResultData { result: value })
    }

    pub fn error(msg: String) -> Self {
        MathResult::Error(ErrorData { error: msg })
    }
}

pub fn decode_request(data: &[u8]) -> Result<Command, Error> {
    let header: Header = serde_json::from_slice(data)?;
    if header.version != PROTOCOL_VERSION {
        return Err(Error::UnsupportedVersion);
    }

    let request: RequestBody = serde_json::from_slice(data)?;
    Ok(request.command)
}

pub fn encode_request(command: Command) -> Result<Vec<u8>, Error> {
    let request = FullRequest {
        version: PROTOCOL_VERSION.to_string(),
        command,
    };
    serde_json::to_vec(&request).map_err(Into::into)
}

pub fn decode_response(data: &[u8]) -> Result<MathResult, Error> {
    let header: Header = serde_json::from_slice(data)?;
    if header.version != PROTOCOL_VERSION {
        return Err(Error::UnsupportedVersion);
    }

    let result: MathResult = serde_json::from_slice(data)?;
    Ok(result)
}

pub fn encode_response(result: MathResult) -> Result<Vec<u8>, Error> {
    let response = FullResponse {
        version: PROTOCOL_VERSION.to_string(),
        result,
    };

    serde_json::to_vec(&response).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_request {
        use super::*;

        #[test]
        fn parse_addition_request() {
            let data = br#"
        {
            "version": "1.0",
            "command": "ADDITION",
            "data": {
                "lhs": 10.0,
                "rhs": 5.0
            }
        }
        "#;

            let result = decode_request(data).expect("request should be valid");

            assert_eq!(
                result,
                Command::Addition(CommandData {
                    lhs: 10.0,
                    rhs: 5.0,
                })
            );
        }

        #[test]
        fn parse_subtraction_request() {
            let data = br#"
        {
            "version": "1.0",
            "command": "SUBTRACTION",
            "data": {
                "lhs": 10.0,
                "rhs": 5.0
            }
        }
        "#;

            let result = decode_request(data).expect("request should be valid");

            assert_eq!(
                result,
                Command::Subtraction(CommandData {
                    lhs: 10.0,
                    rhs: 5.0,
                })
            );
        }

        #[test]
        fn parse_multiplication_request() {
            let data = br#"
        {
            "version": "1.0",
            "command": "MULTIPLICATION",
            "data": {
                "lhs": 10.0,
                "rhs": 5.0
            }
        }
        "#;

            let result = decode_request(data).expect("request should be valid");

            assert_eq!(
                result,
                Command::Multiplication(CommandData {
                    lhs: 10.0,
                    rhs: 5.0,
                })
            );
        }

        #[test]
        fn parse_division_request() {
            let data = br#"
        {
            "version": "1.0",
            "command": "DIVISION",
            "data": {
                "lhs": 10.0,
                "rhs": 5.0
            }
        }
        "#;

            let result = decode_request(data).expect("request should be valid");

            assert_eq!(
                result,
                Command::Division(CommandData {
                    lhs: 10.0,
                    rhs: 5.0,
                })
            );
        }

        #[test]
        fn reject_unsupported_version() {
            let data = br#"
        {
            "version": "2.0",
            "command": "ADDITION",
            "data": {
                "lhs": 10.0,
                "rhs": 5.0
            }
        }
        "#;

            let result = decode_request(data);

            assert!(matches!(result, Err(Error::UnsupportedVersion)));
        }

        #[test]
        fn reject_invalid_json() {
            let data = br#"{ invalid json }"#;

            let result = decode_request(data);

            assert!(matches!(result, Err(Error::Serialize(_))));
        }

        #[test]
        fn reject_unknown_command() {
            let data = br#"
        {
            "version": "1.0",
            "command": "SQUARE_ROOT",
            "data": {
                "lhs": 10.0,
                "rhs": 5.0
            }
        }
        "#;

            let result = decode_request(data);

            assert!(matches!(result, Err(Error::Serialize(_))));
        }

        #[test]
        fn reject_missing_command_data() {
            let data = br#"
        {
            "version": "1.0",
            "command": "ADDITION"
        }
        "#;

            let result = decode_request(data);

            assert!(matches!(result, Err(Error::Serialize(_))));
        }
    }

    mod create_response {
        use super::*;

        #[test]
        fn create_result_response() {
            let response =
                encode_response(MathResult::result(15.0)).expect("response should serialize");
            let response = String::from_utf8(response).expect("response should be valid UTF-8");
            assert_eq!(
                response,
                r#"{"version":"1.0","type":"RESULT","data":{"result":15.0}}"#
            );
        }

        #[test]
        fn create_negative_result_response() {
            let response =
                encode_response(MathResult::result(-15.5)).expect("response should serialize");
            let response = String::from_utf8(response).expect("response should be valid UTF-8");
            assert_eq!(
                response,
                r#"{"version":"1.0","type":"RESULT","data":{"result":-15.5}}"#
            );
        }

        #[test]
        fn create_zero_result_response() {
            let response =
                encode_response(MathResult::result(0.0)).expect("response should serialize");
            let response = String::from_utf8(response).expect("response should be valid UTF-8");
            assert_eq!(
                response,
                r#"{"version":"1.0","type":"RESULT","data":{"result":0.0}}"#
            );
        }

        #[test]
        fn create_error_response() {
            let response = encode_response(MathResult::error("division by zero".to_string()))
                .expect("response should serialize");
            let response = String::from_utf8(response).expect("response should be valid UTF-8");
            assert_eq!(
                response,
                r#"{"version":"1.0","type":"ERROR","data":{"error":"division by zero"}}"#
            );
        }

        #[test]
        fn create_error_response_escapes_special_characters() {
            let response = encode_response(MathResult::error(r#"invalid "operation""#.to_string()))
                .expect("response should serialize");
            let response = String::from_utf8(response).expect("response should be valid UTF-8");
            assert_eq!(
                response,
                r#"{"version":"1.0","type":"ERROR","data":{"error":"invalid \"operation\""}}"#
            );
        }
    }

    mod create_request {
        use super::*;

        #[test]
        fn create_addition_request() {
            let request = encode_request(Command::Addition(CommandData {
                lhs: 10.0,
                rhs: 5.0,
            }))
            .expect("request should serialize");

            let request = String::from_utf8(request).expect("request should be valid UTF-8");

            assert_eq!(
                request,
                r#"{"version":"1.0","command":"ADDITION","data":{"lhs":10.0,"rhs":5.0}}"#
            );
        }
    }

    mod parse_response {
        use super::*;

        #[test]
        fn parse_result_response() {
            let data = br#"
        {
            "version": "1.0",
            "type": "RESULT",
            "data": {
                "result": 15.0
            }
        }
        "#;

            let result = decode_response(data).expect("response should be valid");

            assert_eq!(result, MathResult::result(15.0));
        }
    }
}
