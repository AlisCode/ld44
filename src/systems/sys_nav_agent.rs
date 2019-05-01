use crate::components::{NavAgent, Player, Renderable, Target, TilePos};
use crate::renderer::{LDImages, RenderableType};
use crate::resources::NavMesh;
use specs::prelude::*;

#[derive(Default, Debug)]
pub struct SysNavAgent;

impl<'a> System<'a> for SysNavAgent {
    type SystemData = (
        Read<'a, NavMesh>,
        WriteStorage<'a, NavAgent>,
        ReadStorage<'a, TilePos>,
        WriteStorage<'a, Target>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Renderable>,
    );

    fn run(
        &mut self,
        (nav_mesh, mut nav_agent, tile_pos, mut target, mut player, mut renderable): Self::SystemData,
    ) {
        for (mut nav_agent, tile_pos, target) in (&mut nav_agent, &tile_pos, &mut target).join() {
            // Dont waste time on computation if we don't have a target
            if !nav_agent.has_target() {
                continue;
            }

            if let Some(p) = nav_mesh.next_hop(
                (tile_pos.x, tile_pos.y),
                (nav_agent.target_x.unwrap(), nav_agent.target_y.unwrap()),
            ) {
                target.start_move(p.x, p.y);
                nav_agent.moving = true;
            } else {
                nav_agent.fail();
            }
        }

        for (nav_agent, target, tile_pos, play, mut renderable) in
            (&nav_agent, &target, &tile_pos, &mut player, &mut renderable).join()
        {
            if target.target_x > tile_pos.x {
                renderable.renderable_type = RenderableType::Fixed(LDImages::RobotRight);
            } else if target.target_x < tile_pos.x {
                renderable.renderable_type = RenderableType::Fixed(LDImages::RobotLeft);
            } else if target.target_y < tile_pos.y {
                renderable.renderable_type = RenderableType::Fixed(LDImages::RobotUp);
            } else {
                renderable.renderable_type = RenderableType::Fixed(LDImages::RobotRight);
            }
            if nav_agent.moving {
                play.use_movement();
            }
        }
    }
}
