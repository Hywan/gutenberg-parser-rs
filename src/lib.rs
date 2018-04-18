#![cfg_attr(feature = "wasm", feature(proc_macro, wasm_custom_section, wasm_import_module, global_allocator))]

#[macro_use] extern crate nom;
#[cfg(feature = "wasm")] extern crate wee_alloc;

pub mod ast;
#[macro_use] pub mod combinators;
pub mod parser;

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
