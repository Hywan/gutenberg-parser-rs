use super::Input;
use serde_json as json;

#[derive(Debug, PartialEq, Serialize)]
pub struct Block<'a> {
    pub name: (Input<'a>, Input<'a>),
    pub attributes: Option<json::Value>,
    pub inner_blocks: Vec<Block<'a>>
}
