use crate::ld_entities::EntityDef;
use quicksilver::load_file;
use quicksilver::prelude::*;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::path::Path;

pub struct LevelLoader {
    asset: Asset<String>,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Level {
    curr_level: u32,
}

impl Default for Level {
    fn default() -> Level {
        Level { curr_level: 1 }
    }
}

impl Level {
    pub fn get_url(&self) -> String {
        format!("level/level{}.txt", self.curr_level)
    }
}

impl LevelLoader {
    pub fn load<P: AsRef<Path> + 'static>(path: P) -> LevelLoader {
        let file: Asset<String> = Asset::new(
            load_file(path)
                .and_then(|c| quicksilver::combinators::ok(String::from_utf8(c).unwrap())),
        );

        LevelLoader {
            asset: file,
            done: false,
        }
    }

    pub fn poll(&mut self, world: &mut World) -> bool {
        if self.done {
            return true;
        }

        let res = self.asset.execute(|contents| {
            let defs = ron::de::from_str::<Vec<EntityDef>>(&contents).unwrap();
            defs.into_iter().for_each(|d| {
                d.to_entity(world);
            });
            Ok(())
        });

        if res.is_ok() {
            self.done = true;
            return true;
        }

        false
    }
}
