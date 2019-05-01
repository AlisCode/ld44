use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Cursor;

impl Component for Cursor {
    type Storage = NullStorage<Self>;
}
