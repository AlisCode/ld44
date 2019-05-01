use quicksilver::geom::Vector;
use quicksilver::input::{ButtonState, Mouse, MouseButton};

#[derive(Default)]
pub struct MouseWrapper {
    pub mouse: Option<Mouse>,
}

impl MouseWrapper {
    pub fn get_button(&self, btn: MouseButton) -> ButtonState {
        if let Some(m) = &self.mouse {
            m[btn]
        } else {
            ButtonState::NotPressed
        }
    }

    pub fn get_coords(&self) -> Vector {
        match &self.mouse {
            Some(m) => m.pos(),
            _ => (0, 0).into(),
        }
    }
}
