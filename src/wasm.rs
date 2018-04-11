use super::ast::Block;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./index")]
extern {
    fn accumulate_block(block: Block);
}

#[wasm_bindgen]
pub fn root(input: &str) {
    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
        for block in blocks {
            accumulate_block(block);
        }
    }
}
