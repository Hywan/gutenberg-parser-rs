/*!

Java bindings.

*/

use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::ptr::null;

#[repr(C)]
pub struct NodeSet {
    nodes: Box<[Node]>
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct Node {
    nodeType: usize,
    namespace: *const c_char,
    name: *const c_char,
    attributes: *const c_char,
    content: *const c_char
}

#[no_mangle]
pub extern "C" fn root(_pointer: *const c_char) -> Box<NodeSet> {
    let node = Node {
        nodeType: 0,
        namespace: {
            let namespace = CString::new("foo").unwrap();
            let pointer = namespace.as_ptr();

            mem::forget(namespace);

            pointer
        },
        name: {
            let name = CString::new("bar").unwrap();
            let pointer = name.as_ptr();

            mem::forget(name);

            pointer
        },
        attributes: {
            let attributes = CString::new("{}").unwrap();
            let pointer = attributes.as_ptr();

            mem::forget(attributes);

            pointer
        },
        content: null()
    };

    Box::new(
        NodeSet {
            nodes: vec![node].into_boxed_slice()
        }
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn dropNodeSet(_: Box<NodeSet>) { }

#[no_mangle]
#[allow(non_snake_case)]
pub extern fn dropNode(_: Box<Node>) { }
