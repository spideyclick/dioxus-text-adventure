# Rust-D20

This is an experimental project I created to explore the Rust programming language and various Frameworks.

## Build & Run (Ubuntu)

```sh
sudo apt update
sudo apt upgrade
sudo apt install -y build-essential pkg-config libssl-dev
rustup target add wasm32-unknown-unknown
cargo install dioxus-cli
doxus serve
```

The application should now be running on [localhost:8080](http://localhost:8080)
