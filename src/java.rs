/*!

Java bindings.

*/

use std::ffi::CStr;
use std::os::raw::c_char;

#[repr(C)]
pub struct NodeSet {
    nodes: Box<[Node]>
}

#[repr(C)]
pub enum Node {
    Block {
        namespace: *const c_char,
        name: *const c_char,
        attributes: Option<*const c_char>,
        children: NodeSet
    },
    Phrase(*const c_char)
}

#[no_mangle]
pub extern "C" fn root(pointer: *const c_char) {
    let input = unsafe { CStr::from_ptr(pointer).to_bytes() };

    println!("Hello from Rust: {}", unsafe { ::std::str::from_utf8_unchecked(input) });
}
