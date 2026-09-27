use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub grpc_address: String,
    pub socket_path: String,
}

impl Config {
    pub async fn load(path: &Path) -> Result<Self, String> {
        let data = tokio::fs::read(path)
            .await
            .map_err(|e| format!("Failed to read config: {e}"))?;
        serde_json::from_slice(&data).map_err(|e| format!("Failed to parse config: {e}"))
    }
}
