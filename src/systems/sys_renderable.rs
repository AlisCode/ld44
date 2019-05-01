use crate::components::{Renderable, TilePos};
use crate::renderer::{Renderer, RenderingOrder};
use specs::prelude::*;

pub struct SysRenderable;

impl<'a> System<'a> for SysRenderable {
    type SystemData = (
        Write<'a, Renderer>,
        ReadStorage<'a, TilePos>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, (mut renderer, pos, r): Self::SystemData) {
        for (pos, r) in (&pos, &r).join() {
            renderer.add_order(RenderingOrder {
                pos: pos.rectangle(),
                renderable_type: r.renderable_type.clone(),
                layer: r.layer,
            });
        }
    }
}
