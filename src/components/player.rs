use crate::stuff::{Inventory, LD44Stat};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerInfo {
    pub stats: HashMap<LD44Stat, i32>,
}

impl From<PlayerInfo> for Player {
    fn from(info: PlayerInfo) -> Self {
        Player {
            stats: info.stats.clone(),
            curr_stats: info.stats,
        }
    }
}

pub struct Player {
    pub stats: HashMap<LD44Stat, i32>,
    pub curr_stats: HashMap<LD44Stat, i32>,
}

impl Default for Player {
    fn default() -> Self {
        let mut stats = HashMap::default();
        stats.insert(LD44Stat::BatteryLife, 3);
        stats.insert(LD44Stat::Movement, 1);
        stats.insert(LD44Stat::Armor, 0);

        Player {
            curr_stats: stats.clone(),
            stats,
        }
    }
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}

impl Player {
    pub fn can_still_move(&self) -> bool {
        self.curr_stats[&LD44Stat::Movement] < self.stats[&LD44Stat::Movement]
    }

    pub fn use_movement(&mut self) {
        self.apply_modifier(&LD44Stat::Movement, 1);
    }

    pub fn reset_movement(&mut self) {
        let mov = self.curr_stats.get_mut(&LD44Stat::Movement).unwrap();
        *mov = 0;
    }

    pub fn apply_modifier(&mut self, stat: &LD44Stat, value: i32) {
        let mov = self.curr_stats.get_mut(stat).unwrap();
        *mov += value;
    }

    pub fn apply_inventory(&mut self) {
        if let Some(inv) = Inventory::load_current() {
            inv.items.iter().for_each(|i| {
                i.modifiers.iter().for_each(|m| {
                    self.apply_modifier(&m.0, m.1);
                })
            })
        }
    }
}
