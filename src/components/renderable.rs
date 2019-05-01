use crate::renderer::RenderableType;
use specs::prelude::*;

pub struct Renderable {
    pub visible: bool,
    pub renderable_type: RenderableType,
    pub layer: u32,
}

impl<'a> Component for Renderable {
    type Storage = VecStorage<Self>;
}
