/*!

WebAssembly bindings.

The workflow is basically the following:

  1. Javascript encodes a datum into WebAssembly memory,
  2. Rust reads this datum, runs the parser on it, and encodes the
     resulting AST into a sequence of bytes into WebAssembly memory,
  3. Javascript decodes this sequences of bytes, and maps it to a AST
     in the Javascript lands with specific objects. The respective AST
     are not necessarily the same.

This module is responsible of managing WebAssembly memory allocations
and deallocations, to panic, and to manage an out-of-memory situation.

*/

#![no_std]
#![feature(
    alloc,
    core_intrinsics,
    lang_items,
    panic_implementation,
    proc_macro,
    wasm_custom_section,
    wasm_import_module
)]

extern crate gutenberg_post_parser;
extern crate wee_alloc;
#[macro_use] extern crate alloc;

use gutenberg_post_parser::ast::Node;
use alloc::vec::Vec;
use core::{mem, slice};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::intrinsics::abort();
    }
}

#[lang = "oom"]
#[no_mangle]
pub extern "C" fn oom() -> ! {
    unsafe {
        core::intrinsics::abort();
    }
}

// This is the definition of `std::ffi::c_void`, but wasm runs without std.
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum c_void {
    #[doc(hidden)]
    __variant1,

    #[doc(hidden)]
    __variant2
}

#[no_mangle]
pub extern "C" fn alloc(capacity: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(capacity);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer as *mut c_void
}

#[no_mangle]
pub extern "C" fn dealloc(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn root(pointer: *mut u8, length: usize) -> *mut u8 {
    let input = unsafe { slice::from_raw_parts(pointer, length) };
    let mut output = vec![];

    if let Ok((_remaining, nodes)) = gutenberg_post_parser::root(input) {
        let nodes_length = u32_to_u8s(nodes.len() as u32);

        output.push(nodes_length.0);
        output.push(nodes_length.1);
        output.push(nodes_length.2);
        output.push(nodes_length.3);

        for node in nodes {
            into_bytes(&node, &mut output);
        }
    }

    let pointer = output.as_mut_ptr();
    mem::forget(output);

    pointer
}

fn into_bytes<'a>(node: &Node<'a>, output: &mut Vec<u8>) {
    match *node {
        Node::Block { name, attributes, ref children } => {
            let node_type = 1u8;
            let name_length = name.0.len() + name.1.len() + 1;
            let attributes_length = match attributes {
                Some(attributes) => attributes.len(),
                None             => 4
            };
            let attributes_length_as_u8s = u32_to_u8s(attributes_length as u32);

            let number_of_children = children.len();

            output.push(node_type);
            output.push(name_length as u8);
            output.push(attributes_length_as_u8s.0);
            output.push(attributes_length_as_u8s.1);
            output.push(attributes_length_as_u8s.2);
            output.push(attributes_length_as_u8s.3);
            output.push(number_of_children as u8);

            output.extend(name.0);
            output.push(b'/');
            output.extend(name.1);

            if let Some(attributes) = attributes {
                output.extend(attributes);
            } else {
                output.extend(&b"null"[..]);
            }

            for child in children {
                into_bytes(&child, output);
            }
        },

        Node::Phrase(phrase) => {
            let node_type = 2u8;
            let phrase_length = phrase.len();

            output.push(node_type);

            let phrase_length_as_u8s = u32_to_u8s(phrase_length as u32);

            output.push(phrase_length_as_u8s.0);
            output.push(phrase_length_as_u8s.1);
            output.push(phrase_length_as_u8s.2);
            output.push(phrase_length_as_u8s.3);

            output.extend(phrase);
        }
    }
}

fn u32_to_u8s(x: u32) -> (u8, u8, u8, u8) {
    (
        ((x & 0b_1111_1111_0000_0000_0000_0000_0000_0000_u32) >> 24) as u8,
        ((x & 0b_0000_0000_1111_1111_0000_0000_0000_0000_u32) >> 16) as u8,
        ((x & 0b_0000_0000_0000_0000_1111_1111_0000_0000_u32) >>  8) as u8,
        ( x & 0b_0000_0000_0000_0000_0000_0000_1111_1111_u32       ) as u8
    )
}
