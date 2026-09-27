mod bridge;
mod handler;

use protocol::{Command, CommandData};

#[tokio::main]
async fn main() {
    let bridge = bridge::UnixSocket::new("/tmp/math.sock".to_string());

    let cmd = Command::Addition(CommandData { lhs: 1., rhs: 1. });

    let res = handler::handle(cmd, bridge).await;

    println!("Result {res:?}");
}
