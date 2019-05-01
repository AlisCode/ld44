use crate::components::{Cursor, Target};
use crate::resources::{MouseWrapper, TurnLockMode, TurnLocker};
use quicksilver::input::MouseButton;
use specs::prelude::*;

#[derive(Default)]
pub struct SysCursor;

impl<'a> System<'a> for SysCursor {
    type SystemData = (
        Read<'a, MouseWrapper>,
        Read<'a, TurnLocker>,
        WriteStorage<'a, Target>,
        ReadStorage<'a, Cursor>,
    );

    fn run(&mut self, (mouse, turn_lock, mut target, cursor): Self::SystemData) {
        for (target, _) in (&mut target, &cursor).join() {
            if mouse.get_button(MouseButton::Left).is_down()
                && turn_lock.lock == TurnLockMode::PlayerInput
            {
                let pos = mouse.get_coords();
                target.start_move((pos.x / 32.) as i32, (pos.y / 32.) as i32);
            }
        }
    }
}
