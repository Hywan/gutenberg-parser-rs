use super::ast;
use std::mem;
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);

    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe {
        let _ = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub extern "C" fn dealloc_str(ptr: *mut c_char) {
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

#[no_mangle]
pub extern "C" fn root(data: *mut c_char) -> *mut c_char {
    unsafe {
        let input = CStr::from_ptr(data);
        let mut out = vec![];

        if let Ok((_remaining, blocks)) = super::root(input.to_bytes()) {
            for block in blocks {
                out.extend(block.into_bytes());
            }
        }

        out.push(b'\0');

        CString::from_vec_unchecked(out).into_raw()
    }
}

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
        result.push(b'/');
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
