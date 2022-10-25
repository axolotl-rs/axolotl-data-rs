pub mod blocks;
pub mod item;

#[cfg(test)]
pub mod test {
    use std::path::Path;
    use crate::blocks::{Block, Material, SoundType};
    use crate::item::Item;

    const DATA_LOC: &str = env!("DATA");

    #[test]
    pub fn test() {
        let path = Path::new(DATA_LOC);
        let blocks: Vec<Block> = serde_json::from_reader(std::fs::File::open(path.join("blocks.json")).unwrap()).unwrap();
        println!("{}", tabled::Table::new(&blocks));

        let materials: Vec<Material> = serde_json::from_reader(std::fs::File::open(path.join("materials.json")).unwrap()).unwrap();
        println!("{}", tabled::Table::new(&materials));

        let sounds: Vec<SoundType> = serde_json::from_reader(std::fs::File::open(path.join("soundTypes.json")).unwrap()).unwrap();
        println!("{}", tabled::Table::new(&sounds));

        let items: Vec<Item> = serde_json::from_reader(std::fs::File::open(path.join("items.json")).unwrap()).unwrap();
        let mut generic_items = Vec::new();
        let mut tool_items = Vec::new();
        let mut armor_items = Vec::new();
        let mut block_items = Vec::new();
        let mut tiered_items = Vec::new();
        for item in items.into_iter() {
            match item {
                Item::Generic(generic_item) => generic_items.push(generic_item),
                Item::Tool(tool_item) => tool_items.push(tool_item),
                Item::Armor(armor_item) => armor_items.push(armor_item),
                Item::Block(block_item) => block_items.push(block_item),
                Item::Tiered(tiered_item) => tiered_items.push(tiered_item),
            }
        }
        println!("{}", tabled::Table::new(&generic_items));
        println!("{}", tabled::Table::new(&tool_items));
        println!("{}", tabled::Table::new(&armor_items));
        println!("{}", tabled::Table::new(&block_items));
        println!("{}", tabled::Table::new(&tiered_items));
    }
}