use protocol::Command;

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
