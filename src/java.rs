/*!

Java bindings.

*/

use super::ast;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;
use std::ptr::null;

#[repr(C)]
pub struct NodeSet {
    nodes: Box<[Node]>
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Node {
    nodeType: usize,
    namespace: *const c_char,
    name: *const c_char,
    attributes: *const c_char,
    content: *const c_char
}

#[no_mangle]
pub extern "C" fn root(pointer: *const c_char) -> Box<NodeSet> {
    let input = unsafe { CStr::from_ptr(pointer).to_bytes() };

    if let Ok((_remaining, nodes)) = super::root(input) {
        let nodes: Vec<Node> =
            nodes
                .into_iter()
                .map(
                    |node| {
                        node.into_java()
                    }
                )
                .collect();

        Box::new(
            NodeSet {
                nodes: nodes.into_boxed_slice()
            }
        )
    } else {
        Box::new(NodeSet { nodes: Vec::new().into_boxed_slice() })
    }
}

impl<'a> ast::Node<'a> {
    fn into_java(&self) -> Node {
        match *self {
            ast::Node::Block { name, attributes, children: _ } => {
                Node {
                    nodeType: 0,
                    namespace: {
                        let namespace = CString::new(name.0).unwrap();
                        let pointer = namespace.as_ptr();

                        mem::forget(namespace);

                        pointer
                    },
                    name: {
                        let name = CString::new(name.1).unwrap();
                        let pointer = name.as_ptr();

                        mem::forget(name);

                        pointer
                    },
                    attributes: match attributes {
                        Some(attributes) => {
                            let attributes = CString::new(attributes).unwrap();
                            let pointer = attributes.as_ptr();

                            mem::forget(attributes);

                            pointer
                        },

                        None => {
                            null()
                        }
                    },
                    content: null()
                }
            },

            ast::Node::Phrase(input) => {
                let input = CString::new(input).unwrap();
                let phrase = Node {
                    nodeType: 1,
                    namespace: null(),
                    name: null(),
                    attributes: null(),
                    content: input.as_ptr()
                };

                mem::forget(input);

                phrase
            }
        }
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn dropNodeSet(_: Box<NodeSet>) { }

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn dropNode(_: Box<Node>) { }
