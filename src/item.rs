use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::Tag;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attribute {
    pub attribute: String,
    pub value: Value,
}

pub type Attributes = Vec<Attribute>;
pub type StackSize = u8;
pub type ItemId = usize;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub id: ItemId,
    #[serde(rename = "maxStackSize")]
    pub max_stack_size: StackSize,
    #[serde(rename = "creativeTab")]
    pub creative_tab: Option<String>,
    pub attributes: Attributes,
    pub tags: Vec<Tag>,
}
