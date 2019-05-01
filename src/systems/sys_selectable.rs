use crate::components::{Cursor, Renderable, Selectable, TilePos};
use crate::renderer::{LDImages, RenderableType};
use crate::resources::{MouseWrapper, Selection};
use quicksilver::input::{ButtonState, MouseButton};
use specs::prelude::*;

pub struct SysSelectable;

impl<'a> System<'a> for SysSelectable {
    type SystemData = (
        Write<'a, Selection>,
        Read<'a, MouseWrapper>,
        ReadStorage<'a, TilePos>,
        ReadStorage<'a, Selectable>,
        ReadStorage<'a, Cursor>,
        WriteStorage<'a, Renderable>,
    );

    fn run(
        &mut self,
        (mut selection, mouse, pos, selectable, cursor, mut renderable): Self::SystemData,
    ) {
        if mouse.get_button(MouseButton::Left) != ButtonState::Pressed {
            return;
        }

        selection.just_selected = false;

        let coords = mouse.get_coords();
        let x = coords.x as i32 / 32;
        let y = coords.y as i32 / 32;

        for (pos, s) in (&pos, &selectable).join() {
            if pos.x == x && pos.y == y {
                selection.select(s.info);
                for (_, mut renderable) in (&cursor, &mut renderable).join() {
                    renderable.renderable_type = RenderableType::Fixed(LDImages::CursorSelected);
                }
                return;
            }
        }
    }
}
