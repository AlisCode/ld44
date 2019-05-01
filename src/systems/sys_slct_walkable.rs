use crate::components::{Cursor, NavAgent, Player, Renderable, TilePos, Walkable};
use crate::renderer::{LDImages, RenderableType};
use crate::resources::{MouseWrapper, SelectInfo, Selection, TurnLocker};
use quicksilver::input::{ButtonState, MouseButton};
use specs::prelude::*;

pub struct SysSelectWalkable;

impl<'a> System<'a> for SysSelectWalkable {
    type SystemData = (
        Write<'a, Selection>,
        Read<'a, MouseWrapper>,
        ReadStorage<'a, TilePos>,
        ReadStorage<'a, Walkable>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, NavAgent>,
        Write<'a, TurnLocker>,
        ReadStorage<'a, Cursor>,
        WriteStorage<'a, Renderable>,
    );

    fn run(
        &mut self,
        (
            mut selection,
            mouse,
            pos,
            walk,
            mut play,
            mut nav_agent,
            mut turn_locker,
            curs,
            mut renderable,
        ): Self::SystemData,
    ) {
        if mouse.get_button(MouseButton::Left) != ButtonState::Pressed
            || selection.just_selected
            || selection.select_info != Some(SelectInfo::Player)
        {
            return;
        }

        let coords = mouse.get_coords();
        let x = coords.x as i32 / 32;
        let y = coords.y as i32 / 32;

        for (pos, _) in (&pos, &walk).join() {
            if pos.x == x && pos.y == y {
                for (play, nav_agent) in (&mut play, &mut nav_agent).join() {
                    nav_agent.set_target(x, y);
                    play.reset_movement();
                    turn_locker.next_sys = true;
                }
                return;
            }
        }

        selection.unselect();
        for (_, mut renderable) in (&curs, &mut renderable).join() {
            renderable.renderable_type = RenderableType::Fixed(LDImages::Cursor);
        }
    }
}
