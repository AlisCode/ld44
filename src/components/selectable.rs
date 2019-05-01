use crate::resources::SelectInfo;
use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Selectable {
    pub info: SelectInfo,
}

impl Component for Selectable {
    type Storage = VecStorage<Self>;
}
