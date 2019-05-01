use crate::stuff::LD44Stat;
use quicksilver::saving::load as qs_load;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    Rotors,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub item_type: ItemType,
    pub modifiers: Vec<(LD44Stat, i32)>,
    pub name: String,
    pub desc: String,
    pub icon: String,
}

#[derive(Debug)]
pub struct Inventory {
    pub items: Vec<Item>,
}

impl Inventory {
    pub fn load_current() -> Option<Inventory> {
        Self::load_inv("current_inv")
    }

    pub fn load_owned() -> Option<Inventory> {
        Self::load_inv("owned_inv")
    }

    pub fn load_inv(profile: &str) -> Option<Inventory> {
        qs_load::<InventoryBuilder>("LD44", profile)
            .ok()
            .map(|ib| ib.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InventoryBuilder {
    items: Vec<ItemType>,
}

impl From<InventoryBuilder> for Inventory {
    fn from(i_b: InventoryBuilder) -> Self {
        Inventory {
            items: i_b.items.into_iter().map(Item::get).collect(),
        }
    }
}

impl Item {
    pub fn get(item_type: ItemType) -> Item {
        match item_type {
            ItemType::Rotors => Self::rotors(),
        }
    }

    fn rotors() -> Item {
        Item {
            item_type: ItemType::Rotors,
            modifiers: vec![(LD44Stat::Movement, 1), (LD44Stat::BatteryLife, -2)],
            name: "New rotors".to_owned(),
            desc: "Move faster with these new rotors! Well... If they work ...".to_owned(),
            icon: "".to_owned(),
        }
    }
}
