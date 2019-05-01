use crate::resources::Assets;
use quicksilver::geom::Rectangle;
use quicksilver::graphics::{Background, Color};
use quicksilver::prelude::Window;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LDImages {
    Cursor,
    CursorSelected,
    Ground,
    RobotLeft,
    RobotRight,
    RobotUp,
    LaserOn,
    LaserOff,
    Laser,
}

impl LDImages {
    pub fn get_path(&self) -> &str {
        match self {
            LDImages::Cursor => "Cursor.png",
            LDImages::CursorSelected => "CursorSelected.png",
            LDImages::Ground => "Ground.png",
            LDImages::RobotLeft => "RobotLeft.png",
            LDImages::RobotRight => "RobotRight.png",
            LDImages::RobotUp => "RobotUp.png",
            LDImages::Laser => "Laser.png",
            LDImages::LaserOn => "LaserOn.png",
            LDImages::LaserOff => "LaserOff.png",
            //_ => "",
        }
    }
}

pub struct RenderingOrder {
    pub pos: Rectangle,
    pub renderable_type: RenderableType,
    pub layer: u32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum RenderableType {
    Fixed(LDImages),
    Rotated(LDImages, i32),
    Debug(Color),
}

impl RenderableType {
    pub fn render(&self, assets: &mut Assets, pos: &Rectangle, window: &mut Window) {
        match self {
            RenderableType::Debug(col) => window.draw(pos, Background::Col(*col)),
            RenderableType::Fixed(ldim) => assets.draw_image(window, pos, ldim),
            RenderableType::Rotated(ldim, deg) => {
                assets.draw_image_rotated(window, pos, ldim, *deg)
            }
        }
    }
}

#[derive(Default)]
pub struct Renderer {
    orders: Vec<RenderingOrder>,
    min_layer: u32,
    max_layer: u32,
}

impl Renderer {
    /// Adds an order to the renderer
    pub fn add_order(&mut self, order: RenderingOrder) {
        self.max_layer = self.max_layer.max(order.layer);
        self.min_layer = self.min_layer.min(order.layer);
        self.orders.push(order);
    }

    /// Renders every order on the window, lower layer = higher priority
    pub fn render(&mut self, window: &mut Window, assets: &mut Assets) {
        self.orders.sort_by(|a, b| a.layer.cmp(&b.layer));
        self.orders.iter().for_each(|o| {
            o.renderable_type.render(assets, &o.pos, window);
        });
    }
}

pub struct UIRenderingOrder {
    rectangle: Rectangle,
}

#[derive(Default)]
pub struct UIRenderer {
    orders: Vec<UIRenderingOrder>,
}
