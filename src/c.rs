/*!

C bindings.

*/

use super::root;
use super::ast;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};

#[repr(C)]
pub enum Option_c_char {
    Some(*const c_char),
    None
}

#[repr(C)]
pub enum Node {
    Block {
        namespace: *const c_char,
        name: *const c_char,
        attributes: Option_c_char,
        // Cannot type to `*const Vector_Node` here because of https://github.com/eqrion/cbindgen/issues/43.
        children: *const c_void
    },
    Phrase(*const c_char)
}

#[repr(C)]
pub struct Vector_Node {
    buffer: *const Node,
    length: usize
}

#[repr(C)]
pub enum Result {
    Ok(Vector_Node),
    Err
}

#[no_mangle]
pub extern "C" fn parse(pointer: *const c_char) -> Result {
    let input = unsafe { CStr::from_ptr(pointer).to_bytes() };

    if let Ok((_remaining, nodes)) = root(input) {
        let output: Vec<Node> =
            nodes
                .into_iter()
                .map(
                    |node| {
                        match node {
                            ast::Node::Block { name, attributes, children } => {
                                Node::Block {
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
                                            let some = Option_c_char::Some(attributes.as_ptr());

                                            mem::forget(attributes);

                                            some
                                        },

                                        None => {
                                            Option_c_char::None
                                        }
                                    },
                                    children: {
                                        let vector = Vector_Node {
                                            buffer: vec![].as_slice().as_ptr(),
                                            length: 42,
                                        };

                                        let boxed_vector = Box::new(vector);
                                        let static_ref_vector: &'static mut Vector_Node = Box::leak(boxed_vector);

                                        let vector_ptr = static_ref_vector as *const _ as *const c_void;

                                        vector_ptr
                                    }
                                }
                            },

                            ast::Node::Phrase(input) => {
                                let input = CString::new(input).unwrap();
                                let phrase = Node::Phrase(input.as_ptr());

                                mem::forget(input);

                                phrase
                            }
                        }
                    }
                )
                .collect();

        Result::Ok(
            Vector_Node {
                buffer: output.as_slice().as_ptr(),
                length: output.len()
            }
        )
    } else {
        Result::Err
    }
}
