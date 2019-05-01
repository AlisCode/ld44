use crate::components::Player;
use specs::prelude::*;

pub struct SysPlayerUI {}

impl<'a> System<'a> for SysPlayerUI {
    type SystemData = (Write<'a, UIRenderer>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut ui_renderer, play): Self::SystemData) {}
}
