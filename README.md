# ECE421 Project 3

## Run Locally

### 1) Install `wasm-pack`

Install by running `cargo install wasm-pack`

### 2) Build WebAssembly

This command will build the WebAssembly target and bundle it in the `pkg/` directory

`wasm-pack build --target web`

### 3) Bundle JavaScript

This command will bundle the WebAssembly with the entry point function defined in `main.js`

`rollup ./main.js --format iife --file ./pkg/bundle.js`

### 4) Launch Server

`cargo run --bin project3`
