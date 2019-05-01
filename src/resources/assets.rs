use crate::renderer::LDImages;
use quicksilver::geom::Transform;
use quicksilver::graphics::{Background, Color, Image};
use quicksilver::prelude::{Asset, Rectangle, Window};
use std::collections::HashMap;

#[derive(Default)]
pub struct Assets {
    asset_map: HashMap<LDImages, Asset<Image>>,
}

impl Assets {
    pub fn add_image(&mut self, ld_image: LDImages) {
        let a = Asset::new(Image::load(ld_image.get_path().to_owned()));
        self.asset_map.insert(ld_image.clone(), a);
    }

    pub fn draw_image(&mut self, window: &mut Window, pos: &Rectangle, ld_image: &LDImages) {
        if let Some(a) = self.asset_map.get_mut(ld_image) {
            let _ = a.execute(|img| {
                window.draw(pos, Background::Img(img));
                Ok(())
            });
        } else {
            window.draw(pos, Background::Col(Color::BLACK));
        }
    }

    pub fn draw_image_rotated(
        &mut self,
        window: &mut Window,
        pos: &Rectangle,
        ld_image: &LDImages,
        deg: i32,
    ) {
        if let Some(a) = self.asset_map.get_mut(ld_image) {
            let _ = a.execute(|img| {
                window.draw_ex(pos, Background::Img(img), Transform::rotate(deg), 1);
                Ok(())
            });
        } else {
            window.draw(pos, Background::Col(Color::BLACK));
        }
    }
}
