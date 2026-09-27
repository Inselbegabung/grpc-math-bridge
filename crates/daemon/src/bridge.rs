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
            unix_listener: initialize_socket(socket_path).await,
            socket_path: PathBuf::from(socket_path),
        })
    }

    pub async fn run(&self, shutdown: impl Future<Output = ()>) {
        tokio::pin!(shutdown);

        loop {
            tokio::select! {
                result = self.unix_listener.accept() => {
                    match result {
                        Ok((_stream, socket)) => {
                            debug!("Connection from: {socket:?}");
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

async fn initialize_socket(socket_path: &str) -> UnixListener {
    if tokio::fs::try_exists(socket_path)
        .await
        .expect("working file system")
    {
        tokio::fs::remove_file(socket_path)
            .await
            .expect("expect clean system");
    }

    UnixListener::bind(socket_path).expect("unix listener binding")
}
