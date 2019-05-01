use specs::prelude::*;

#[derive(Debug, Default)]
pub struct WorldTurn;

impl Component for WorldTurn {
    type Storage = NullStorage<Self>;
}