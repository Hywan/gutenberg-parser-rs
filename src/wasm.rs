use serde_json as json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn root(input: &str) -> String {
    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
        match json::to_string(&blocks) {
            Ok(output) => output,
            Err(_) => String::from("")
        }
    } else {
        String::from("")
    }
}
