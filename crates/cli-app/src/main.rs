mod args;
mod bridge;
mod handler;

use args::Args;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cmd = args.command().unwrap_or_else(|err| {
        eprintln!("Input expression is not valid: {err}");
        std::process::exit(1);
    });

    let bridge = bridge::UnixSocket::new(args.socket_path);

    let res = handler::handle(cmd, bridge).await;

    println!("Result {res:?}");
}
