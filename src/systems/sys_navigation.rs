use crate::components::{TilePos, Walkable};
use crate::resources::NavMesh;
use specs::prelude::*;

pub struct SysNavigation;

impl<'a> System<'a> for SysNavigation {
    type SystemData = (
        Write<'a, NavMesh>,
        WriteStorage<'a, Walkable>,
        ReadStorage<'a, TilePos>,
    );

    fn run(&mut self, (mut nav_mesh, mut walk, pos): Self::SystemData) {
        for (mut walk, pos) in (&mut walk, &pos).join() {
            match walk.id_nav_mesh {
                Some(id) => nav_mesh.update(id, &pos),
                _ => nav_mesh.setup(&mut walk, &pos),
            }
        }
    }
}
