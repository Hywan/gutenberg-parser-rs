// Increase the macro recursion limit.
#![recursion_limit="128"]

#![feature(proc_macro, wasm_custom_section, wasm_import_module, global_allocator)]

#[macro_use]
extern crate nom;
extern crate wasm_bindgen;
extern crate wee_alloc;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

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
named_attr!(
    #[doc="Test"],
    pub block_list,
    tag!("<!-- wp:foo /-->")
);

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
