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
    alloc_error_handler,
    core_intrinsics,
    lang_items
)]

extern crate gutenberg_post_parser;
extern crate wee_alloc;
#[macro_use] extern crate alloc;

use gutenberg_post_parser::ast::Node;
use alloc::vec::Vec;
use core::{mem, slice, str::from_utf8_unchecked};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        core::intrinsics::abort();
    }
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
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
pub extern "C" fn alloc(capacity: usize) -> *mut u8 {
    let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    pointer
}

#[no_mangle]
pub extern "C" fn dealloc_u8(pointer: *mut u8, capacity: usize) {
    unsafe {
        let _: Vec<u8> = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_u32(pointer: *mut u32, capacity: usize) {
    unsafe {
        let _: Vec<u32> = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn root(pointer: *mut u8, length: usize) -> *mut u32 {
    let input = unsafe { slice::from_raw_parts(pointer, length) };
    let mut output = vec![0; 1];

    if let Ok((_remaining, nodes)) = gutenberg_post_parser::root(input) {
        let nodes_length = nodes.len() as u32;
        let mut remaining_input = input;
        let mut utf16_offset = 0;

        output.reserve(nodes_length as usize * 12);
        output.push(nodes_length);

        for node in nodes {
            remaining_input = into_bytes(&node, &remaining_input, &mut utf16_offset, &mut output);
        }
    }

    output[0] = output.len() as u32;

    let pointer = output.as_mut_ptr();

    mem::forget(output);

    pointer
}

fn into_bytes<'a>(node: &Node<'a>, mut remaining_input: &'a [u8], utf16_offset: &mut u32, output: &mut Vec<u32>) -> &'a [u8] {
    match *node {
        Node::Block { name, attributes, ref children } => {
            let node_type = 1u32;
            
            output.push(node_type);

            let name_length = name.0.len() + name.1.len() + 1;

            output.push(name_length as u32);
            output.extend(name.0.iter().map(|c: &u8| *c as u32));
            output.push(b'/' as u32);
            output.extend(name.1.iter().map(|c: &u8| *c as u32));

            let input_offset: usize = remaining_input.offset(&name.1) + name.1.len() + 1;
            remaining_input = &remaining_input[input_offset..];
            *utf16_offset += input_offset as u32;

            let attributes_offset;
            let attributes_length;

            match attributes {
                Some(attributes) => {
                    let mut input_offset: usize = remaining_input.offset(&attributes);

                    *utf16_offset += input_offset as u32;
                    attributes_offset = *utf16_offset;
                    attributes_length = count_utf16(unsafe { from_utf8_unchecked(attributes) });
                    *utf16_offset += attributes_length;

                    input_offset += attributes.len();
                    remaining_input = &remaining_input[input_offset..];
                },

                None => {
                    attributes_offset = 0;
                    attributes_length = 0;
                }
            };

            output.push(attributes_offset);
            output.push(attributes_length);
            output.push(children.len() as u32);

            for child in children {
                remaining_input = into_bytes(&child, remaining_input, utf16_offset, output);
            }

            remaining_input
        },

        Node::Phrase(phrase) => {
            let node_type = 2u32;

            output.push(node_type);

            let mut input_offset: usize = remaining_input.offset(&phrase);

            *utf16_offset += input_offset as u32;
            let phrase_offset = *utf16_offset;
            let phrase_length = count_utf16(unsafe { from_utf8_unchecked(phrase) });
            *utf16_offset += phrase_length;

            input_offset += phrase.len();
            remaining_input = &remaining_input[input_offset..];

            output.push(phrase_offset);
            output.push(phrase_length);

            remaining_input
        }
    }
}

trait Offset {
    fn offset(&self, second: &Self) -> usize;
}

impl<'a> Offset for &'a [u8] {
    fn offset(&self, second: &Self) -> usize {
        second.as_ptr() as usize - self.as_ptr() as usize
    }
}

fn count_utf16(input: &str) -> u32 {
    input.chars().fold(
        0,
        |acc, ch| {
            let code = ch as u32;

            if (code & 0xffff) == code {
                acc + 1
            } else {
                acc + 2
            }
        }
    )
}
