#![feature(proc_macro, wasm_custom_section, wasm_import_module, global_allocator)]

#[macro_use]
extern crate nom;
//extern crate wasm_bindgen;
extern crate wee_alloc;
extern crate regex;
#[cfg(test)] #[macro_use] extern crate serde_json;
#[cfg(not(test))] extern crate serde_json;

//use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

mod ast;
#[macro_use]
mod combinators;
mod parser;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


/// Test
//#[wasm_bindgen]
pub fn root(input: &str) -> bool {
    if let Ok(_) = parser::block_list(input.as_bytes()) {
        true
    } else {
        false
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        assert_eq!(root("<!-- wp:foo /-->"), true);
    }
}
