/*!

WebAssembly bindings.

*/

use super::ast::Block;
use alloc::Vec;
use core::{self, mem, slice};

// This is required by `wee_alloc` and `no_std`.
#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_args: core::fmt::Arguments, _file: &'static str, _line: u32) -> ! {
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

    return pointer as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn root(pointer: *mut u8, length: usize) -> *mut u8 {
    unsafe {
        let input = slice::from_raw_parts(pointer, length);
        let mut output = vec![];

        if let Ok((_remaining, blocks)) = super::root(input) {
            for block in blocks {
                output.extend(block.into_bytes());
            }
        }

        let pointer = output.as_mut_ptr();
        mem::forget(output);

        pointer
    }
}

impl<'a> Block<'a> {
    fn into_bytes(&self) -> Vec<u8> {
        let name = self.name;
        let name_length = name.0.len() + name.1.len() + 1;

        let attributes = self.attributes;
        let attributes_length = match attributes {
            Some(attributes) => attributes.len(),
            None             => 2
        };

        let inner_blocks: Vec<u8> =
            self.inner_blocks
                .iter()
                .flat_map(
                    |ref block| {
                        block.into_bytes()
                    }
                )
                .collect();
        let inner_blocks_length = inner_blocks.len();

        let mut output = Vec::with_capacity(3 + name_length + attributes_length + inner_blocks_length);

        output.push(name_length as u8);
        output.push(attributes_length as u8);
        output.push(inner_blocks_length as u8);

        output.extend(name.0);
        output.push(b'/');
        output.extend(name.1);

        if let Some(attributes) = attributes {
            output.extend(attributes);
        } else {
            output.extend(&b"{}"[..]);
        }

        output.extend(inner_blocks);

        output
    }
}
