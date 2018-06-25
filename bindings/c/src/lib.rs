/*!

C bindings.

The workflow is basically the following:

  1. In `build.rs`, `cbindgen` is run, and scans the code to generate a
     C header file,
  2. `rustc` compiles the code into a static library,
  3. `clang` compiles a C program that preferably uses the C header
     and links with the static library to generate a “C binary”.

This module is responsible to map the AST into a C
representation. Data are duplicated (most of them are stored on the
stack).

*/

extern crate gutenberg_post_parser;

use gutenberg_post_parser::root;
use gutenberg_post_parser::ast;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub enum Option_c_char {
    Some(*const c_char),
    None
}

#[repr(C)]
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub struct Vector_Node {
    buffer: *const Node,
    length: usize
}

#[repr(C)]
#[derive(Debug, PartialEq)]
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
                        into_c(&node)
                    }
                )
                .collect();

        let vector_node = Vector_Node {
            buffer: output.as_slice().as_ptr(),
            length: output.len()
        };

        mem::forget(output);

        Result::Ok(vector_node)
    } else {
        Result::Err
    }
}

fn into_c<'a>(node: &ast::Node<'a>) -> Node {
    match *node {
        ast::Node::Block { name, attributes, ref children } => {
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
                    let output: Vec<Node> =
                        children
                        .into_iter()
                        .map(
                            |node| {
                                into_c(&node)
                            }
                        )
                        .collect();

                    let vector = if output.is_empty() {
                        Box::new(
                            Vector_Node {
                                buffer: ptr::null(),
                                length: 0
                            }
                        )
                    } else {
                        Box::new(
                            Vector_Node {
                                buffer: output.as_slice().as_ptr(),
                                length: output.len()
                            }
                        )
                    };
                    let vector_ptr = Box::into_raw(vector) as *const _ as *const c_void;

                    mem::forget(output);

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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! str_to_c_char {
        ($input:expr) => (
            {
                ::std::ffi::CString::new($input).unwrap()
            }
        )
    }

    macro_rules! c_char_to_str {
        ($input:ident) => (
            {
                unsafe { ::std::ffi::CStr::from_ptr(*$input) }.to_str().unwrap()
            }
        )
    }

    #[test]
    fn test_root_with_a_phrase() {
        let input = str_to_c_char!("foo");
        let output = parse(input.as_ptr());

        match output {
            Result::Ok(result) => match result {
                Vector_Node { buffer, length } if length == 1 => match unsafe { &*buffer } {
                    Node::Phrase(phrase) => {
                        assert_eq!(c_char_to_str!(phrase), "foo");
                    },

                    _ => assert!(false)
                },

                _ => assert!(false)
            },

            _ => assert!(false)
        }
    }

    #[test]
    fn test_root_with_a_block() {
        let input = str_to_c_char!("<!-- wp:foo {bar} /-->");
        let output = parse(input.as_ptr());

        match output {
            Result::Ok(result) => match result {
                Vector_Node { buffer, length } if length == 1 => match unsafe { &*buffer } {
                    Node::Block { namespace, name, attributes, children } => {
                        assert_eq!(c_char_to_str!(namespace), "core");
                        assert_eq!(c_char_to_str!(name), "foo");

                        match attributes {
                            Option_c_char::Some(attributes) => {
                                assert_eq!(c_char_to_str!(attributes), "{bar}");
                            },

                            _ => assert!(false)
                        }

                        let children = unsafe { &*(children as *const _ as *const Vector_Node) };

                        assert_eq!(children.length, 0);
                    },

                    _ => assert!(false)
                },

                _ => assert!(false)
            },

            _ => assert!(false)
        }
    }

    #[test]
    fn test_root_with_a_block_with_no_attributes() {
        let input = str_to_c_char!("<!-- wp:foo /-->");
        let output = parse(input.as_ptr());

        match output {
            Result::Ok(result) => match result {
                Vector_Node { buffer, length } if length == 1 => match unsafe { &*buffer } {
                    Node::Block { namespace, name, attributes, children } => {
                        assert_eq!(c_char_to_str!(namespace), "core");
                        assert_eq!(c_char_to_str!(name), "foo");

                        match attributes {
                            Option_c_char::None => assert!(true),
                            _ => assert!(false)
                        }

                        let children = unsafe { &*(children as *const _ as *const Vector_Node) };

                        assert_eq!(children.length, 0);
                    },

                    _ => assert!(false)
                },

                _ => assert!(false)
            },

            _ => assert!(false)
        }
    }

    #[test]
    fn test_root_with_a_block_with_specific_namespace() {
        let input = str_to_c_char!("<!-- wp:foo/bar /-->");
        let output = parse(input.as_ptr());

        match output {
            Result::Ok(result) => match result {
                Vector_Node { buffer, length } if length == 1 => match unsafe { &*buffer } {
                    Node::Block { namespace, name, attributes, children } => {
                        assert_eq!(c_char_to_str!(namespace), "foo");
                        assert_eq!(c_char_to_str!(name), "bar");

                        match attributes {
                            Option_c_char::None => assert!(true),
                            _ => assert!(false)
                        }

                        let children = unsafe { &*(children as *const _ as *const Vector_Node) };

                        assert_eq!(children.length, 0);
                    },

                    _ => assert!(false)
                },

                _ => assert!(false)
            },

            _ => assert!(false)
        }
    }
}
