use super::ast;
use std::str;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./parser_definitions")]
extern {
    fn accumulate_block(block: Block);

    type Block;
    #[wasm_bindgen(constructor)]
    fn new(block_as_json: Vec<u8>) -> Block;
}

#[wasm_bindgen]
pub fn root(input: &str) {
    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
        for block in blocks {
            accumulate_block(
                Block::new(
                    block.into_bytes()
                )
            );
        }
    }
}

impl<'a> ast::Block<'a> {
    fn into_bytes(&self) -> Vec<u8> {
        let mut result = vec![];

        let name = self.name;
        let attributes = self.attributes;
        let inner_blocks: Vec<u8> =
            self.inner_blocks
                .iter()
                .flat_map(
                    |ref block| {
                        block.into_bytes()
                    }
                )
                .collect();

        result.push((name.0.len() + name.1.len() + 1) as u8);
        result.push(
            match attributes {
                Some(attributes) => attributes.len() as u8,
                None             => 2u8
            }
        );
        result.push(inner_blocks.len() as u8);

        result.extend(name.0);
        result.push('/' as u8);
        result.extend(name.1);

        if let Some(attributes) = attributes {
            result.extend(attributes);
        } else {
            result.extend(&b"{}"[..]);
        }

        result.extend(inner_blocks);

        result
    }
}
