/*!

The Abstract Syntax Tree (AST), i.e. the output of the parser.

*/

use super::{Input, InputElement};
use std::marker::PhantomData;
use std::vec::Vec;

/// Represent a node in an AST.
#[derive(PartialEq)]
#[cfg_attr(not(feature = "no_std"), derive(Debug))]
pub enum Node<'a, I, T>
where
    I: 'a + InputElement,
    T: Input<'a, I>
{
    /// A block is the elementary component of the post format.
    Block {
        /// The fully-qualified block name, where the left part of the
        /// pair represents the namespace, and the right part of the pair
        /// represents the block name.
        name: (T, T),

        /// A block can have attributes, just like an HTML element can
        /// have attributes. Attributes are encoded as a JSON string.
        attributes: Option<T>,

        /// A block can have inner blocks or phrases.
        children: Vec<Node<'a, I, T>>,

        phantom: PhantomData<&'a I>
    },

    /// Anything that is not a block.
    Phrase(T)
}
