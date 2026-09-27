use crate::handler::Handle;
use std::path::PathBuf;
use tokio::net::UnixListener;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{UnixStream, unix::OwnedReadHalf},
};
use tracing::{debug, error};

pub struct UnixSocket {
    socket_path: PathBuf,
    unix_listener: UnixListener,
}

impl UnixSocket {
    pub async fn new(socket_path: &str) -> Result<Self, String> {
        Ok(Self {
            unix_listener: initialize_socket(socket_path).await?,
            socket_path: PathBuf::from(socket_path),
        })
    }

    pub async fn run<H>(&self, handler: H, shutdown: impl Future<Output = ()>)
    where
        H: Handle + Clone + Send + Sync + 'static,
    {
        tokio::pin!(shutdown);

        loop {
            tokio::select! {
                result = self.unix_listener.accept() => {
                    match result {
                        Ok((stream, socket)) => {
                            debug!("Connection from: {socket:?}");

                            let mut inner_handler = handler.clone();

                            tokio::spawn(async move {
                                if let Err(err) =
                                    handle_connection(
                                        stream,
                                        &mut inner_handler,
                                    ).await
                                {
                                    error!("Connection failed: {err}");
                                }
                            });
                        }

                        Err(err) => {
                            error!("Failed to accept connection: {err}");
                        }
                    }
                }

                _ = &mut shutdown => {
                    break;
                }
            }
        }
    }
}

impl Drop for UnixSocket {
    fn drop(&mut self) {
        if let Err(error) = std::fs::remove_file(&self.socket_path) {
            tracing::debug!(
                "Failed to remove Unix socket '{}': {error}",
                self.socket_path.display()
            );
        }
    }
}

async fn initialize_socket(socket_path: &str) -> Result<UnixListener, String> {
    if tokio::fs::try_exists(socket_path)
        .await
        .map_err(|e| format!("Socket existence couldn't be checked, error: {e}."))?
    {
        return Err(format!(
            "The socket '{socket_path}' already exists, remove it or select an other socket."
        ));
    }

    UnixListener::bind(socket_path).map_err(|e| format!("Socket binding failed, error: {e}."))
}

pub async fn handle_connection(
    stream: UnixStream,
    handler: &mut impl Handle,
) -> Result<(), String> {
    let (reader, mut writer) = stream.into_split();

    let bytes = read_request(reader).await?;

    let response_bytes = handler.handle(bytes).await;

    writer
        .write_all(&response_bytes)
        .await
        .map_err(|e| e.to_string())?;

    writer.write_all(b"\n").await.map_err(|e| e.to_string())?;

    Ok(())
}

async fn read_request(stream: OwnedReadHalf) -> Result<Vec<u8>, String> {
    const MAX_REQUEST_SIZE: u64 = 4096;

    let mut data = Vec::new();
    let reader = BufReader::new(stream);

    let bytes_read = reader
        .take(MAX_REQUEST_SIZE + 1)
        .read_until(b'\n', &mut data)
        .await
        .map_err(|e| e.to_string())?;

    if bytes_read > MAX_REQUEST_SIZE as usize {
        return Err("Unix socket message too large.".into());
    }

    if !data.ends_with(b"\n") {
        return Err("Received data is not `\n` terminated.".into());
    }

    Ok(data)
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::handler::MockHandle;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    #[tokio::test]
    async fn initialize_socket_creates_unix_socket() {
        let temp_dir = tempfile::tempdir().expect("create temp directory");

        let socket_path = temp_dir.path().join("math.sock");

        assert!(!socket_path.exists());

        let _listener = initialize_socket(
            socket_path
                .to_str()
                .expect("socket path should be valid UTF-8"),
        )
        .await;

        assert!(socket_path.exists());
    }

    #[tokio::test]
    async fn initialize_socket_fails_if_socket_already_exists() {
        let temp_dir = tempfile::tempdir().expect("create temp directory");
        let socket_path = temp_dir.path().join("math.sock");

        let socket_path = socket_path
            .to_str()
            .expect("socket path should be valid UTF-8");

        let listener = initialize_socket(socket_path)
            .await
            .expect("initialize first socket");

        let result = initialize_socket(socket_path).await;

        assert!(result.is_err());

        drop(listener);
    }

    #[tokio::test]
    async fn handle_connection_forwards_request_to_handler() {
        let (client, server) = UnixStream::pair().expect("create unix stream pair");

        let mut handler = MockHandle::new();

        handler
            .expect_handle()
            .with(mockall::predicate::eq(b"request\n".to_vec()))
            .times(1)
            .returning(|_| b"response".to_vec());

        let server_task = tokio::spawn(async move {
            handle_connection(server, &mut handler)
                .await
                .expect("handle connection");
        });

        let (reader, mut writer) = client.into_split();

        writer.write_all(b"request\n").await.expect("write request");

        let mut reader = BufReader::new(reader);
        let mut response = String::new();

        reader
            .read_line(&mut response)
            .await
            .expect("read response");

        assert_eq!(response, "response\n");

        server_task.await.expect("server task");
    }

    #[tokio::test]
    async fn read_request_reads_newline_terminated_message() {
        let (stream, mut peer) = UnixStream::pair().expect("create socket pair");
        let (reader, _) = stream.into_split();

        peer.write_all(b"request\n").await.expect("write request");

        let result = read_request(reader).await.expect("read request");

        assert_eq!(result, b"request\n");
    }

    #[tokio::test]
    async fn read_request_rejects_message_without_newline() {
        let (stream, mut peer) = UnixStream::pair().expect("create socket pair");
        let (reader, _) = stream.into_split();

        peer.write_all(b"request").await.expect("write request");
        peer.shutdown().await.expect("shutdown peer");

        let result = read_request(reader).await;

        assert_eq!(result, Err("Received data is not `\n` terminated.".into()));
    }

    #[tokio::test]
    async fn read_request_rejects_message_larger_than_limit() {
        const MAX_REQUEST_SIZE: usize = 4096;

        let (stream, mut peer) = UnixStream::pair().expect("create socket pair");
        let (reader, _) = stream.into_split();

        let request = vec![b'x'; MAX_REQUEST_SIZE + 1];

        peer.write_all(&request).await.expect("write request");

        let result = read_request(reader).await;

        assert_eq!(result, Err("Unix socket message too large.".into()));
    }
}
