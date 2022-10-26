use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub mod blocks;
pub mod item;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    #[serde(flatten)]
    pub properties: HashMap<String, Value>,
}

#[cfg(test)]
pub mod test {
    use crate::blocks::{Block, Material, SoundType};
    use crate::item::Item;
    use std::fmt::Debug;
    use std::io::Write;
    use std::path::Path;

    const DATA_LOC: &str = env!("DATA");

    #[test]
    pub fn test() {
        let path = Path::new(DATA_LOC);
        let blocks: Vec<Block> =
            serde_json::from_reader(std::fs::File::open(path.join("blocks.json")).unwrap())
                .unwrap();
        test_print(blocks, "blocks.test.output");
        let materials: Vec<Material> =
            serde_json::from_reader(std::fs::File::open(path.join("materials.json")).unwrap())
                .unwrap();
        test_print(materials, "materials.test.output");
        let sounds: Vec<SoundType> =
            serde_json::from_reader(std::fs::File::open(path.join("soundTypes.json")).unwrap())
                .unwrap();
        test_print(sounds, "soundTypes.test.output");
        let items: Vec<Item> =
            serde_json::from_reader(std::fs::File::open(path.join("items.json")).unwrap()).unwrap();
        test_print(items, "items.test.output");
    }

    fn test_print(blocks: Vec<impl Debug>, file: &str) {
        let test_path = Path::new(file);
        let mut file = std::fs::File::create(test_path).unwrap();
        for block in blocks {
            writeln!(file, "{:#?}", block).unwrap();
        }
    }
}
