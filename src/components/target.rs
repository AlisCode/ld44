use specs::prelude::*;

#[derive(Default, Debug)]
pub struct Target {
    pub target_x: i32,
    pub target_y: i32,
    // speed in tiles / seconds
    pub speed: f32,
    pub reached: bool,
}

impl Component for Target {
    type Storage = VecStorage<Self>;
}

impl Target {
    pub fn new(x: i32, y: i32, speed: f32) -> Self {
        Target {
            target_x: x,
            target_y: y,
            speed,
            reached: true,
        }
    }

    pub fn start_move(&mut self, x: i32, y: i32) {
        self.target_x = x;
        self.target_y = y;
        self.reached = false;
    }
}
