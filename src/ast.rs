/*!

The Abstract Syntax Tree (AST), i.e. the output of the parser.

*/

use super::Input;
#[cfg(feature = "no_std")] use alloc::Vec;

/// Represent a node in an AST.
#[derive(PartialEq)]
#[cfg_attr(not(feature = "no_std"), derive(Debug))]
pub enum Node<'a> {
    /// A block is the elementary component of the post format.
    Block {
        /// The fully-qualified block name, where the left part of the
        /// pair represents the namespace, and the right part of the pair
        /// represents the block name.
        name: (Input<'a>, Input<'a>),

        /// A block can have attributes, just like an HTML element can
        /// have attributes. Attributes are encoded as a JSON string.
        attributes: Option<Input<'a>>,

        /// A block can have inner blocks or phrases.
        children: Vec<Node<'a>>
    },

    /// Anything that is not a block.
    Phrase(Input<'a>)
}
