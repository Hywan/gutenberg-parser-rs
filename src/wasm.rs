use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn root(input: &str) -> bool {
    if let Ok(_) = super::root(input.as_bytes()) {
        true
    } else {
        false
    }
}
