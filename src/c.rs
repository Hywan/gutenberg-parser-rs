/*!

C bindings.

*/

use nom;
use super::Input;
use super::root;
use super::ast;

#[repr(C)]
pub struct Foo {
    bar: u8
}

#[no_mangle]
pub extern "C" fn parse(input: *mut u8) -> Foo {
    Foo { bar: b' ' }
}
