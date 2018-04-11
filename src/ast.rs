#[allow(unused_imports)]
use super::Input;
use serde_json as json;

#[cfg(not(feature = "wasm"))]
#[derive(Debug, PartialEq, Serialize)]
pub struct Block<'a> {
    pub name: (Input<'a>, Input<'a>),
    pub attributes: Option<json::Value>,
    pub inner_blocks: Vec<Block<'a>>
}

#[cfg(feature = "wasm")]
#[derive(Debug, PartialEq, Serialize)]
pub struct Block {
    pub name: (String, String),
    pub attributes: Option<json::Value>,
    pub inner_blocks: Vec<Block>
}
