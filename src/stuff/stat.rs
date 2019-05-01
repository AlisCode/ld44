use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LD44Stat {
    BatteryLife,
    Movement,
    Armor,
}
