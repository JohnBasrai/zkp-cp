# Chaum-Pedersen Zero Knowledge Proof
# gRPC client/server for authentication

Rust crates directly used

- tokio, Rusts asynchronous runtime.

  Tokio is a tool that provides the building blocks for writing network
  applications without compromising speed. It includes a stack of
  various components, such as Hyper, Tonic, Tower, and Mio, for
  different needs and scenarios.

- tonic   For gRPC, Rust implementation of gRPC
- clap    Argument parsing
- anyhow  Provides better error handling

## Building

```
cargo build --release --bin client --bin server
```

## Running

In one shell window run command
```
cargo run --release --bin server
```
In a second shell window run command
```
cargo run --release --bin client -- --user "UserName" --password "Password"
```
You shouldn't crate a real program that takes password on the command line but this is a demo only.

## Containerization

Work in progress.

