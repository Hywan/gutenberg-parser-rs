/*!

C bindings.

*/

use super::root;
use super::ast;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_char;

/// cbindgen:derive-helper-methods
#[repr(C)]
pub enum Option_c_char {
    Some(*const c_char),
    None
}

/// cbindgen:derive-helper-methods
#[repr(C)]
pub enum Node {
    Block {
        namespace: *const c_char,
        name: *const c_char,
        attributes: Option_c_char,
    },
    Phrase(*const c_char)
}

#[repr(C)]
pub struct Vector_Node {
    buffer: *const Node,
    length: usize
}

/// cbindgen:derive-helper-methods
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
                                let name_0 = CString::new(name.0).unwrap();
                                let name_1 = CString::new(name.1).unwrap();

                                let block = Node::Block {
                                    namespace: name_0.as_ptr(),
                                    name: name_1.as_ptr(),
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
                                    }
                                };

                                mem::forget(name_0);
                                mem::forget(name_1);

                                block
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
