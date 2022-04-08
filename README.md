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

### 4) Connect to the website

Open your browser and navigate to the address Trunk is serving at (can be found in CLI output, default is localhost:8080)

## Run Server Locally

### 1) Install SQLite3

Using whatever method you prefer for your OS
(make sure it is in your PATH)

### 2) Launch Local Server

Run this command to launch the local development server

```sh
cargo run -p prj3_server
```

If you have no set up database, or migrations need to be run either launch the CLI (see below) or use this command

```sh
cargo run -p prj3_server --features build_database
```

### 3) Connect to Server from Backend

To use the database with the local dev server ensure that the proxy configuration in `Trunk.toml` is configured correctly (should be configured correctly assuming server is served on `localhost:8000` which is the default port)

## Run CLI

### 1) Install SQLite3 (see run server locally)

### 2) Run CLI

```sh
cargo run -p prj3_cli
```

## Perform Database Changes

Database changes are performed using the Diesel CLI visit the Diesel [Getting Started](https://diesel.rs/guides/getting-started.html) guide for information on installing the Diesel CLI and creating/performing migrations

## Initialize Database

The database can be built by either running the CLI (where the specified database path will be initialized with a newly configured database) or by running the server with the `build_database` feature enabled.
