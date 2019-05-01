use specs::prelude::*;

#[derive(Default)]
pub struct Walkable {
    pub id_nav_mesh: Option<u64>,
}

impl Component for Walkable {
    type Storage = VecStorage<Self>;
}
