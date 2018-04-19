#![cfg_attr(feature = "wasm", no_std)]
#![
    cfg_attr(
        feature = "wasm",
        feature(
            proc_macro,
            wasm_custom_section,
            wasm_import_module,
            global_allocator,
            alloc,
            core_intrinsics,
            lang_items
        )
    )
]


#[cfg(feature = "wasm")] #[macro_use] extern crate alloc;
#[macro_use] extern crate nom;
#[cfg(feature = "wasm")] extern crate wee_alloc;


#[cfg(feature = "wasm")]
use alloc::Vec;


pub mod ast;
#[macro_use] pub mod combinators;
pub mod parser;
#[cfg(feature = "wasm")] pub mod wasm;


#[cfg(feature = "wasm")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


/// Represent the type of the input elements.
pub type InputElement = u8;

/// Represent the type of the input.
pub type Input<'a> = &'a [InputElement];

/// Test
pub fn root(input: Input) -> Result<(Input, Vec<ast::Block>), nom::Err<Input>> {
    parser::block_list(input)
}
