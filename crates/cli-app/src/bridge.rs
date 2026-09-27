use crate::handler::Bridge;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{
        UnixStream,
        unix::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub struct UnixSocket {
    socket_path: String,
}

impl UnixSocket {
    pub fn new(socket_path: String) -> Self {
        Self { socket_path }
    }
}

#[async_trait::async_trait]
impl Bridge for UnixSocket {
    async fn send(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
        let (reader, writer) = open_socket(&self.socket_path).await;
        write_message(writer, &data).await?;
        read_message(reader).await
    }
}

async fn open_socket(socket_path: &str) -> (OwnedReadHalf, OwnedWriteHalf) {
    let stream = UnixStream::connect(socket_path)
        .await
        .map_err(|e| e.to_string())
        .expect("open unix socket");

    stream.into_split()
}

async fn write_message(mut writer: OwnedWriteHalf, data: &[u8]) -> Result<(), String> {
    writer
        .write_all(&data)
        .await
        .map_err(|e| e.to_string())
        .map_err(|e| e.to_string())?;
    writer
        .write_all(b"\n")
        .await
        .map_err(|e| e.to_string())
        .map_err(|e| e.to_string())?;
    Ok(())
}

async fn read_message(reader: OwnedReadHalf) -> Result<Vec<u8>, String> {
    let mut reader = BufReader::new(reader);

    let mut response = Vec::new();

    let bytes_read = reader
        .read_until(b'\n', &mut response)
        .await
        .map_err(|e| e.to_string())?;

    if bytes_read == 0 {
        return Err("Daemon closed connection without response".into());
    }

    if response.last() == Some(&b'\n') {
        response.pop();
    }

    Ok(response)
}
