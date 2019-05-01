use crate::components::*;
use crate::renderer::{LDImages, RenderableType};
use crate::resources::SelectInfo;
use quicksilver::saving::load as qs_load;
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum LDEntitiesType {
    Ground,
    Player,
}

#[derive(Debug, Deserialize)]
pub struct EntityDef {
    pub entity_type: LDEntitiesType,
    pub args: HashMap<String, String>,
}

impl EntityDef {
    pub fn to_entity(self, world: &mut World) -> Entity {
        match self.entity_type {
            LDEntitiesType::Ground => {
                let x: i32 = self.args["x"].parse::<i32>().unwrap();
                let y: i32 = self.args["y"].parse::<i32>().unwrap();
                LDEntities::ground(world, x, y)
            }
            LDEntitiesType::Player => {
                let x: i32 = self.args["x"].parse::<i32>().unwrap();
                let y: i32 = self.args["y"].parse::<i32>().unwrap();
                LDEntities::player(world, x, y)
            }
        }
    }
}

pub struct LDEntities;

impl LDEntities {
    pub fn cursor(world: &mut World) -> Entity {
        world
            .create_entity()
            .with(TilePos::new(0, 0))
            .with(Renderable {
                visible: true,
                renderable_type: RenderableType::Fixed(LDImages::Cursor),
                layer: 2,
            })
            .with(Target {
                target_x: 0,
                target_y: 0,
                speed: 10.,
                reached: false,
            })
            .with(Cursor)
            .build()
    }

    pub fn ground(world: &mut World, x: i32, y: i32) -> Entity {
        world
            .create_entity()
            .with(TilePos::new(x, y))
            .with(Renderable {
                visible: true,
                renderable_type: RenderableType::Fixed(LDImages::Ground),
                layer: 0,
            })
            .with(Walkable::default())
            .build()
    }

    pub fn player(world: &mut World, x: i32, y: i32) -> Entity {
        let play: Player = qs_load::<PlayerInfo>("LD44", "player_info")
            .ok()
            .map(|p| p.into())
            .unwrap_or_default();

        world
            .create_entity()
            .with(TilePos::new(x, y))
            .with(Target::new(x, y, 5.))
            .with(Renderable {
                visible: true,
                renderable_type: RenderableType::Fixed(LDImages::RobotRight),
                layer: 1,
            })
            .with(NavAgent::default())
            .with(Selectable {
                info: SelectInfo::Player,
            })
            .with(play)
            .build()
    }
}
