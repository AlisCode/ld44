use quicksilver::geom::{Rectangle, Shape};
use quicksilver::graphics::{Background, FontStyle, Image};
use quicksilver::input::{Mouse, MouseButton};
use quicksilver::lifecycle::Window;

pub struct UIButton {
    pub text: String,
    pub rectangle: Rectangle,
    pub rectangle_text: Rectangle,
    pub font_style: FontStyle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UIAction {
    Pressed,
    Hovered,
    NotPressed,
}

impl UIAction {
    pub fn render_button(
        &self,
        normal: &Image,
        hovered: &Image,
        clicked: &Image,
        rectangle: &Rectangle,
        window: &mut Window,
    ) {
        match self {
            UIAction::NotPressed => window.draw(rectangle, Background::Img(normal)),
            UIAction::Hovered => window.draw(rectangle, Background::Img(hovered)),
            UIAction::Pressed => window.draw(rectangle, Background::Img(clicked)),
        }
    }
}

impl UIButton {
    pub fn new(
        font_style: FontStyle,
        txt: &'static str,
        rectangle: Rectangle,
        rectangle_text: Rectangle,
    ) -> UIButton {
        UIButton {
            text: txt.into(),
            rectangle,
            rectangle_text,
            font_style,
        }
    }

    pub fn render_txt(&self) -> (&Rectangle, &str) {
        (&self.rectangle_text, &self.text)
    }

    pub fn render_box(&self, mouse: Mouse) -> UIAction {
        if self.rectangle.contains(mouse.pos()) {
            if mouse[MouseButton::Left].is_down() {
                return UIAction::Pressed;
            } else {
                return UIAction::Hovered;
            }
        }
        UIAction::NotPressed
    }
}
