use demoinfocs_rs::parser::Parser;
use serde::Serialize;
use std::io::Cursor;
use wasm_bindgen::prelude::*;
use console_error_panic_hook;

#[wasm_bindgen]
pub fn parse_demo(data: &[u8]) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();
    let mut parser = Parser::new(Cursor::new(data));
    parser
        .parse_header()
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    parser
        .parse_to_end()
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

    let participants = parser.game_state().participants();
    let stats: Vec<PlayerInfo> = participants
        .connected()
        .iter()
        .map(|p| PlayerInfo {
            name: p.name.clone(),
        })
        .collect();

    serde_wasm_bindgen::to_value(&stats).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[derive(Serialize)]
struct PlayerInfo {
    name: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("Build with --target wasm32-unknown-unknown to generate WebAssembly");
}

#[cfg(target_arch = "wasm32")]
fn main() {}

