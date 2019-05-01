use crate::components::{Target, TilePos};
use crate::resources::DeltaTime;
use specs::prelude::*;

#[derive(Default, Debug)]
pub struct SysMoving;

impl<'a> System<'a> for SysMoving {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, TilePos>,
        WriteStorage<'a, Target>,
    );

    fn run(&mut self, (delta_time, mut pos, mut tar): Self::SystemData) {
        for (mut pos, mut tar) in (&mut pos, &mut tar).join() {
            if tar.reached {
                continue;
            }

            let delta_x = (tar.target_x * 32) as f32 - pos.real_x;
            let delta_y = (tar.target_y * 32) as f32 - pos.real_y;

            let val = delta_time.delta * tar.speed;
            pos.real_x += delta_x * val;
            pos.real_y += delta_y * val;

            if (pos.real_x.round() as i32 - (tar.target_x * 32)).abs() < 5
                && (pos.real_y.round() as i32 - (tar.target_y * 32)).abs() < 5
            {
                pos.real_x = (tar.target_x * 32) as f32;
                pos.real_y = (tar.target_y * 32) as f32;
                pos.x = tar.target_x;
                pos.y = tar.target_y;
                tar.reached = true;
            }
        }
    }
}
