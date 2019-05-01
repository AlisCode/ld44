use crate::components::Target;
use specs::prelude::*;

#[derive(Default)]
pub struct NavAgent {
    pub target_x: Option<i32>,
    pub target_y: Option<i32>,
    pub moving: bool,
    pub failed_flag: bool,
}

impl Component for NavAgent {
    type Storage = VecStorage<Self>;
}

impl NavAgent {
    pub fn has_target(&self) -> bool {
        self.target_x.is_some() && self.target_y.is_some()
    }

    pub fn fail(&mut self) {
        self.target_x = None;
        self.target_y = None;
        self.failed_flag = true;
    }

    pub fn reached(&self, target: &Target) -> bool {
        match (self.target_x, self.target_y) {
            (Some(x), Some(y)) if x != target.target_x || y != target.target_y => false,
            _ => true,
        }
    }

    pub fn set_target(&mut self, x: i32, y: i32) {
        self.target_x = Some(x);
        self.target_y = Some(y);
    }
}
