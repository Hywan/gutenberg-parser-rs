use super::Input;
#[cfg(feature = "wasm")] use alloc::Vec;

#[derive(PartialEq)]
#[cfg_attr(not(feature = "wasm"), Debug)]
pub struct Block<'a> {
    pub name: (Input<'a>, Input<'a>),
    pub attributes: Option<Input<'a>>,
    pub inner_blocks: Vec<Block<'a>>
}
