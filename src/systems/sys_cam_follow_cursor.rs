use crate::components::{Cursor, TilePos};
use crate::resources::Camera;
use quicksilver::geom::Shape;
use specs::prelude::*;

pub struct SysCamFollowCursor;

impl<'a> System<'a> for SysCamFollowCursor {
    type SystemData = (
        Write<'a, Camera>,
        ReadStorage<'a, TilePos>,
        ReadStorage<'a, Cursor>,
    );

    fn run(&mut self, (mut camera, tile_pos, cursor): Self::SystemData) {
        for (tile_pos, _) in (&tile_pos, &cursor).join() {
            camera.view = camera
                .view
                .with_center((tile_pos.real_x + 16., tile_pos.real_y + 16.));
        }
    }
}
