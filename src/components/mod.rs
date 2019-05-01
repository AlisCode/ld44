use specs::prelude::*;

mod cursor;
mod nav_agent;
mod player;
mod renderable;
mod selectable;
mod target;
mod tile_pos;
mod walkable;
mod world_turn;

pub use cursor::Cursor;
pub use nav_agent::NavAgent;
pub use player::{Player, PlayerInfo};
pub use renderable::Renderable;
pub use selectable::Selectable;
pub use target::Target;
pub use tile_pos::TilePos;
pub use walkable::Walkable;
pub use world_turn::WorldTurn;

pub fn register_components(world: &mut World) {
    world.register::<TilePos>();
    world.register::<Target>();
    world.register::<Renderable>();
    world.register::<Cursor>();
    world.register::<Player>();
    world.register::<Walkable>();
    world.register::<NavAgent>();
    world.register::<WorldTurn>();
    world.register::<Selectable>();
}
