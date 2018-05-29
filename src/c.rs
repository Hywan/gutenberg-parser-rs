/*!

C bindings.

*/

use super::root;
use std::ffi::CStr;
use std::os::raw::c_char;

/// cbindgen:derive-helper-methods
#[repr(C)]
pub enum Result {
    Ok(Vec<Node>),
    Err
}

/// cbindgen:derive-helper-methods
#[repr(C)]
pub enum Node {
    Block {
        name: *const c_char,
        attributes: Option<*const c_char>,
        children: Vec<Node>
    },
    Phrase(*const c_char)
}

#[no_mangle]
pub extern "C" fn parse(pointer: *const c_char) -> Result {
    let input = unsafe { CStr::from_ptr(pointer).to_bytes() };

    if let Ok((_remaining, _nodes)) = root(input) {
        Result::Ok(vec![Node::Phrase(b' ' as *const c_char)])
    } else {
        Result::Err
    }
}
