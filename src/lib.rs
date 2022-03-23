// Yew wasm-pack setup referenced from https://github.com/yewstack/yew-wasm-pack-minimal
mod client;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    yew::start_app::<client::app::App>();

    Ok(())
}