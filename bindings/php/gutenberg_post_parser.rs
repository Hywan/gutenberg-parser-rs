#![crate_type = "staticlib"]

use std::os::raw::c_char;
use std::mem;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn gutenberg_post_parser(input: *const c_char) -> *const c_char {
    let input = unsafe { CStr::from_ptr(input).to_bytes() };

    let mut string = unsafe { String::from_utf8_unchecked(input.to_vec()) };
    string.push('!');

    let cstring = CString::new(string).unwrap();
    let pointer = cstring.as_ptr();

    mem::forget(cstring);

    pointer
}
