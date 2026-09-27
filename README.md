# grpc-math-bridge

A Rust-based gRPC math service with a Unix domain socket bridge and CLI client.

## Toolchain

The following tools are required to work on this project:

- Rust and Cargo
  - Rust version `1.98.1`, as specified in `rust-toolchain.toml`
- `protoc` (https://protobuf.dev/downloads/) version `35.0`

## Crates

### Server

The server crate contains the gRPC server.

To run the server:

```bash
cargo run --bin server
```

The server provides a CLI interface for configuring the host IP and port. Run:

```bash
cargo run --bin server -- -h
```

to display the available options.

The host and port can also be configured using environment variables:

- `MATH_GRPC_HOST` — the host IP address
- `MATH_GRPC_PORT` — the host port

These environment variables are used when they are set and no corresponding CLI arguments are specified.

CLI arguments take precedence over environment variables.


## Quality Gates

The project uses the following quality gates:

- Check code formatting:
  ```bash
  cargo fmt --check
  ```

- Run Clippy:
  ```bash
  cargo clippy
  ```

- Check for unused dependencies:
  ```bash
  cargo machete
  ```

- All tests muss succeed:
  ```bash
  cargo test
  ```

## Logging

All crates use `tracing` for logging. The log level can be configured using the `RUST_LOG` environment variable.

For example, to enable `info`-level logging when running the server:

```bash
RUST_LOG=info cargo run --bin server
```

Other log levels, such as `debug`, `warn`, or `error`, can be configured in the same way.