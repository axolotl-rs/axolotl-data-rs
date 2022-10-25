use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
#[cfg(feature = "tabled")]
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Attribute {
    pub attribute: String,
    pub value: Value,
}

pub type Attributes = Vec<Attribute>;

pub trait ItemType {
    fn get_name(&self) -> &str;

    fn get_id(&self) -> i64;
    fn get_max_stack_size(&self) -> i64;
    fn get_attributes(&self) -> &Attributes;
}
macro_rules! define_item_type {
    (
        $ty:ty,
        $name:ident,
        $id:ident,
        $max_stack_size:ident,
        $attributes:ident
    ) => {
        impl ItemType for $ty {
            #[inline]
            fn get_name(&self) -> &str {
                &self.$name
            }
            #[inline]
            fn get_id(&self) -> i64 {
                self.$id
            }
            #[inline]
            fn get_max_stack_size(&self) -> i64 {
                self.$max_stack_size
            }
            #[inline]
            fn get_attributes(&self) -> &Attributes {
                &self.$attributes
            }
        }
    };
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub enum Item {
    Generic(GenericItem),
    Tool(ToolItem),
    Armor(ArmorItem),
    Block(BlockItem),
    Tiered(TieredItem),
}

impl ItemType for Item {
    fn get_name(&self) -> &str {
        match self {
            Item::Generic(item) => item.get_name(),
            Item::Tool(item) => item.get_name(),
            Item::Armor(item) => item.get_name(),
            Item::Block(item) => item.get_name(),
            Item::Tiered(item) => item.get_name(),
        }
    }

    fn get_id(&self) -> i64 {
        match self {
            Item::Generic(item) => item.get_id(),
            Item::Tool(item) => item.get_id(),
            Item::Armor(item) => item.get_id(),
            Item::Block(item) => item.get_id(),
            Item::Tiered(item) => item.get_id(),
        }
    }

    fn get_max_stack_size(&self) -> i64 {
        match self {
            Item::Generic(item) => item.get_max_stack_size(),
            Item::Tool(item) => item.get_max_stack_size(),
            Item::Armor(item) => item.get_max_stack_size(),
            Item::Block(item) => item.get_max_stack_size(),
            Item::Tiered(item) => item.get_max_stack_size(),
        }
    }

    fn get_attributes(&self) -> &Attributes {
        match self {
            Item::Generic(item) => item.get_attributes(),
            Item::Tool(item) => item.get_attributes(),
            Item::Armor(item) => item.get_attributes(),
            Item::Block(item) => item.get_attributes(),
            Item::Tiered(item) => item.get_attributes(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct GenericItem {
    name: String,
    id: i64,
    #[serde(rename = "maxStackSize")]
    max_stack_size: i64,
    #[serde(rename = "creativeTab")]
    #[cfg_attr(feature = "tabled", tabled(skip))]
    creative_tab: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    attributes: Attributes,
}
define_item_type!(GenericItem, name, id, max_stack_size, attributes);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct BlockItem {
    name: String,
    block: String,
    id: i64,
    #[serde(rename = "maxStackSize")]
    max_stack_size: i64,
    #[serde(rename = "creativeTab")]
    #[cfg_attr(feature = "tabled", tabled(skip))]
    creative_tab: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]
    attributes: Attributes,
}
define_item_type!(BlockItem, name, id, max_stack_size, attributes);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct TieredItem {
    tier: String,
    name: String,
    id: i64,
    #[serde(rename = "maxStackSize")]
    max_stack_size: i64,
    #[serde(rename = "creativeTab")]
    #[cfg_attr(feature = "tabled", tabled(skip))]

    creative_tab: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]

    attributes: Attributes,
}
define_item_type!(TieredItem, name, id, max_stack_size, attributes);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct ArmorItem {
    #[serde(rename = "armorSlot")]
    armor_slot: String,
    material: String,
    name: String,
    id: i64,
    #[serde(rename = "maxStackSize")]
    max_stack_size: i64,
    #[serde(rename = "creativeTab")]
    #[cfg_attr(feature = "tabled", tabled(skip))]

    creative_tab: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]

    attributes: Attributes,
}
define_item_type!(ArmorItem, name, id, max_stack_size, attributes);
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "tabled", derive(tabled::Tabled))]
pub struct ToolItem {
    #[serde(rename = "toolType")]
    tool_type: String,
    #[serde(rename = "blockTag")]
    block_tag: String,
    tier: String,
    name: String,
    id: usize,
    #[serde(rename = "maxStackSize")]
    max_stack_size: u8,
    #[serde(rename = "creativeTab")]
    #[cfg_attr(feature = "tabled", tabled(skip))]

    creative_tab: Option<String>,
    #[cfg_attr(feature = "tabled", tabled(skip))]

    attributes: Attributes,
}

define_item_type!(ToolItem, name, id, max_stack_size, attributes);
