// Increase the macro recursion limit.
#![recursion_limit="128"]

#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[macro_use]
extern crate nom;

use nom::IResult;

/// Test
#[wasm_bindgen]
pub fn root(input: &str) -> bool {
    if let Ok(_) = block_list(input.as_bytes()) {
        true
    } else {
        false
    }
}

/// Test
fn block_list(input: &[u8]) -> IResult<&[u8], &[u8], u32> {
    tag!(input, "<!-- wp:foo /-->")
}

#[cfg(test)]
mod tests {
    use super::{root, block_list};

    #[test]
    fn test_root() {
        assert_eq!(root("<!-- wp:foo /-->"), true);
    }

    #[test]
    fn test_block_list() {
        let input = &b"<!-- wp:foo /-->"[..];
        let output = Ok((&b""[..], input));

        assert_eq!(block_list(input), output);
    }
}
