use quicksilver::geom::Rectangle;
use quicksilver::graphics::View;

pub struct Camera {
    pub view: Rectangle,
}

impl Default for Camera {
    fn default() -> Self {
        let view = Rectangle::new((0, 0), (320, 240));
        Camera { view }
    }
}

impl Camera {
    pub fn as_view(&self) -> View {
        View::new(self.view)
    }
}
