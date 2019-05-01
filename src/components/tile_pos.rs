use quicksilver::geom::Rectangle;
use specs::{Component, VecStorage};

const TILE_SIZE: i32 = 32;

pub struct TilePos {
    pub x: i32,
    pub y: i32,
    pub real_x: f32,
    pub real_y: f32,
}

impl Component for TilePos {
    type Storage = VecStorage<Self>;
}

impl TilePos {
    pub fn rectangle(&self) -> Rectangle {
        Rectangle::new(
            (self.real_x as i32, self.real_y as i32),
            (TILE_SIZE, TILE_SIZE),
        )
    }

    pub fn new(x: i32, y: i32) -> Self {
        TilePos {
            x,
            y,
            real_x: (x * TILE_SIZE) as f32,
            real_y: (y * TILE_SIZE) as f32,
        }
    }
}
