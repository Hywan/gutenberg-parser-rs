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

macro_rules! push_u32_as_u8s {
    ($u32:ident in $output:ident) => (
        $output.push((($u32 >> 24) & 0xff) as u8);
        $output.push((($u32 >> 16) & 0xff) as u8);
        $output.push((($u32 >>  8) & 0xff) as u8);
        $output.push((($u32      ) & 0xff) as u8);
    )
}

#[no_mangle]
pub extern "C" fn root(pointer: *mut u8, length: usize) -> *mut u8 {
    let input = unsafe { slice::from_raw_parts(pointer, length) };
    let mut output = vec![0; 4];

    output.reserve(length);

    if let Ok((_remaining, nodes)) = gutenberg_post_parser::root(input) {
        let nodes_length = nodes.len() as u32;
        let mut utf16_offset = 0;
        let mut remaining_input = input;

        push_u32_as_u8s!(nodes_length in output);

        for node in nodes {
            remaining_input = into_bytes(&node, &remaining_input, &mut utf16_offset, &mut output);
        }
    }

    let output_length = output.len() as u32;

    output[0] = ((output_length >> 24) & 0xff) as u8;
    output[1] = ((output_length >> 16) & 0xff) as u8;
    output[2] = ((output_length >>  8) & 0xff) as u8;
    output[3] = ((output_length      ) & 0xff) as u8;

    let pointer = output.as_mut_ptr();

    mem::forget(output);

    pointer
}

fn into_bytes<'a>(node: &Node<'a>, mut remaining_input: &'a [u8], utf16_offset: &mut u32, output: &mut Vec<u8>) -> &'a [u8] {
    match *node {
        Node::Block { name, attributes, ref children } => {
            // Push node type.
            output.push(1u8);

            let name_length = name.0.len() + name.1.len() + 1;

            output.push(name_length as u8);
            output.extend(name.0);
            output.push(b'/');
            output.extend(name.1);

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
                    attributes_length = unsafe { from_utf8_unchecked(attributes) }.encode_utf16().count() as u32;
                    *utf16_offset += attributes_length;

                    input_offset += attributes.len();
                    remaining_input = &remaining_input[input_offset..];
                },

                None => {
                    attributes_offset = 0;
                    attributes_length = 0;
                }
            };

            push_u32_as_u8s!(attributes_offset in output);
            push_u32_as_u8s!(attributes_length in output);

            let number_of_children = children.len();

            output.push(number_of_children as u8);

            for child in children {
                remaining_input = into_bytes(&child, remaining_input, utf16_offset, output);
            }

            remaining_input
        },

        Node::Phrase(phrase) => {
            // Push node type.
            output.push(2u8);

            let mut input_offset: usize = remaining_input.offset(&phrase);

            *utf16_offset += input_offset as u32;
            let phrase_offset = *utf16_offset;
            let phrase_length = unsafe { from_utf8_unchecked(phrase) }.encode_utf16().count() as u32;
            *utf16_offset += phrase_length;

            input_offset += phrase.len();
            remaining_input = &remaining_input[input_offset..];

            // Push phrase.
            push_u32_as_u8s!(phrase_offset in output);
            push_u32_as_u8s!(phrase_length in output);

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
