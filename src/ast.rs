/*!

The Abstract Syntax Tree (AST), i.e. the output of the parser.

*/

use super::Input;
#[cfg(feature = "wasm")] use alloc::Vec;

/// A block is the elementary component of the post format.
#[derive(PartialEq)]
#[cfg_attr(not(feature = "wasm"), derive(Debug))]
pub struct Block<'a> {
    /// The fully-qualified block name, where the left part of the
    /// pair represents the namespace, and the right part of the pair
    /// represents the block name.
    pub name: (Input<'a>, Input<'a>),

    /// A block can have attributes, just like an HTML element can
    /// have attributes. Attributes are encoded as a JSON string.
    pub attributes: Option<Input<'a>>,

    /// A block can have inner blocks, just like an HTML element can
    /// have inner HTML elements.
    pub inner_blocks: Vec<Block<'a>>
}
