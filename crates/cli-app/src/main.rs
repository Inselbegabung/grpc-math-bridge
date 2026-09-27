mod args;
mod bridge;
mod handler;

use args::Args;
use clap::Parser;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cmd = args.command().unwrap_or_else(|err| {
        eprintln!("Input error - {err}");
        std::process::exit(1);
    });

    let bridge = bridge::UnixSocket::new(args.socket_path);

    let result = handler::handle(cmd, bridge).await.unwrap_or_else(|err| {
        eprintln!("Request failed - {err}");
        std::process::exit(1);
    });

    print_result(result);
}

fn print_result(result: protocol::MathResult) {
    match result {
        protocol::MathResult::Result(data) => println!("{}", data.result),
        protocol::MathResult::Error(data) => eprintln!("Error: {}", data.error),
    }
}
