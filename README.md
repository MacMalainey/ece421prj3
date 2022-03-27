# ECE421 Project 3

## Run Client Locally

### 1) Install WebAssembly Target

```sh
rustup target add wasm32-unknown-unknown
```

### 2) Install Trunk

Trunk is the tool used for managing deployment and packaging

```sh
# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
cargo install trunk
cargo install wasm-bindgen-cli
```

### 3) Launch Local Development Web Server

```sh
trunk serve
```

## Run Server Locally

### 1) Install SQLite3

Using whatever method you prefer for your OS
(make sure it is in your PATH)

### 2) Launch Local Server

```sh
cargo run -p prj3_server
```

## Run CLI

### 1) Install SQLite3 (see run server locally)

### 2) Run CLI

```sh
cargo run -p prj3_cli
```

## Perform Database Changes

Database changes are performed using the Diesel CLI visit the Diesel [Getting Started](https://diesel.rs/guides/getting-started.html) guide for information on installing the Diesel CLI and creating/performing migrations