use serde_json as json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn root(input: &str) -> String {
    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
        json::to_string(&blocks).unwrap()
    } else {
        "".to_owned()
    }
}
