use super::ast;
use std::{mem, str};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

extern {
    fn accumulate_block(block: Block);
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);

    return pointer as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(pointer: *mut c_void, capacity: usize) {
    unsafe {
        let _buffer = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern "C" fn root(data: *mut c_char) -> *mut c_char {
    unsafe {
        let input = CStr::from_ptr(data);
        let out = vec![];

        if let Ok((_remaining, blocks)) = super::root(input.to_bytes()) {
            for block in blocks {
                out.expend(block.into_bytes());
            }
        }

        CString::from_vec_unchecked(out).into_raw()
    }
}

//#[wasm_bindgen]
//pub fn root(input: &str) {
//    if let Ok((_remaining, blocks)) = super::root(input.as_bytes()) {
//        for block in blocks {
//            accumulate_block(
//                Block::new(
//                    block.into_bytes()
//                )
//            );
//        }
//    }
//}

impl<'a> ast::Block<'a> {
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

        let mut result = Vec::with_capacity(3 + name_length + attributes_length + inner_blocks_length);

        result.push(name_length as u8);
        result.push(attributes_length as u8);
        result.push(inner_blocks_length as u8);

        result.extend(name.0);
        result.push('/' as u8);
        result.extend(name.1);

        if let Some(attributes) = attributes {
            result.extend(attributes);
        } else {
            result.extend(&b"{}"[..]);
        }

        result.extend(inner_blocks);

        result
    }
}
