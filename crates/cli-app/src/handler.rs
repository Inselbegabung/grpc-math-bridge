use protocol::Command;

#[cfg_attr(test, mockall::automock)]
#[async_trait::async_trait]
pub trait Bridge {
    async fn send(&self, data: Vec<u8>) -> Result<Vec<u8>, String>;
}

pub async fn handle(command: Command, bridge: impl Bridge) -> Result<protocol::MathResult, String> {
    let request_data =
        protocol::create_request(command).map_err(|e| format!("Parse request failed '{e:?}'"))?;
    let response_data = bridge
        .send(request_data)
        .await
        .map_err(|e| format!("Send request failed '{e:?}'"))?;
    protocol::parse_response(&response_data).map_err(|e| format!("Parse request failed '{e:?}'"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::eq;
    use protocol::CommandData;

    #[tokio::test]
    async fn handle_request() {
        let mut bridge = MockBridge::new();

        bridge
            .expect_send()
            .with(eq(
                br#"{"version":"1.0","command":"ADDITION","data":{"lhs":1.0,"rhs":2.0}}"#.to_vec(),
            ))
            .times(1)
            .returning(|_| {
                Ok(br#"{"version":"1.0","type":"RESULT","data":{"result":3.0}}"#.to_vec())
            });

        let result = handle(
            Command::Addition(CommandData { lhs: 1.0, rhs: 2.0 }),
            bridge,
        )
        .await
        .expect("request should succeed");

        assert_eq!(result, protocol::MathResult::result(3.0));
    }
}
