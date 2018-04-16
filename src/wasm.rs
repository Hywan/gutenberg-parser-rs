use super::ast;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde_json as json;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./parser_definitions")]
extern {
    fn accumulate_block(block: Block);

    type Block;
    #[wasm_bindgen(constructor)]
    fn new(block_as_json: String) -> Block;
}

#[wasm_bindgen]
pub fn root(input: &str) {
    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
        for block in blocks {
            accumulate_block(
                block.into_js_block()
            );
        }
    }
}

impl<'a> Serialize for ast::Block<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let mut state = serializer.serialize_struct("Block", 3)?;

        state.serialize_field(
            "name",
            &(
                unsafe { str::from_utf8_unchecked(&self.name.0) },
                unsafe { str::from_utf8_unchecked(&self.name.1) }
            )
        )?;
        state.serialize_field("attributes", &self.attributes)?;
        state.serialize_field("inner_blocks", &self.inner_blocks)?;

        state.end()
    }
}

impl<'a> ast::Block<'a> {
    fn into_js_block(&self) -> Block {
        Block::new(
            json::to_string(self).unwrap()
        )
    }
}
