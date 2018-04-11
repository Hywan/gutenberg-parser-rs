use serde_json as json;

#[derive(Debug, PartialEq)]
pub struct Block<'a> {
    pub name: (&'a [u8], &'a [u8]),
    pub attributes: Option<json::Value>,
    pub inner_blocks: Vec<Block<'a>>
}
