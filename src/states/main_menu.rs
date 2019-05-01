use crate::level::Level;
use crate::states::main_state::LD44States;
use crate::ui::{UIAction, UIButton};
use quicksilver::geom::Rectangle;
use quicksilver::graphics::{Background, Color, FontStyle, Image};
use quicksilver::input::{ButtonState, Key};
use quicksilver::lifecycle::{State, Window};
use quicksilver::prelude::{Asset, Font, Future, Result};
use quicksilver::saving::save as qs_save;

pub struct MainMenu {
    quit: bool,
    ui_image: Asset<Image>,
    ui_button: Asset<(Image, Image, Image)>,
    ui_font: Asset<Font>,
    begin_btn: UIButton,
}

impl State for MainMenu {
    fn new() -> Result<Self> {
        let ui_font = Asset::new(Font::load("font.ttf"));
        let ui_image = Asset::new(Image::load("title_screen/title.png"));
        let ui_button = Asset::new(Future::join3(
            Image::load("ui/Button.png"),
            Image::load("ui/ButtonHovered.png"),
            Image::load("ui/ButtonClicked.png"),
        ));
        let begin_btn = UIButton::new(
            FontStyle::new(18., Color::BLACK),
            "Begin",
            Rectangle::new((305, 300), (50, 24)),
            Rectangle::new((315, 303), (30, 18)),
        );

        let level = Level::default();
        qs_save("LD44", "curr_level", &level)?;

        Ok(MainMenu {
            ui_font,
            ui_image,
            ui_button,
            begin_btn,
            quit: false,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Space] == ButtonState::Pressed {
            self.quit = true;
        }
        Ok(())
    }

    /// awful hard-coded menu
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        let rect = &self.begin_btn.rectangle;
        let action = self.begin_btn.render_box(window.mouse());
        if action == UIAction::Pressed {
            self.quit = true;
        }
        self.ui_button.execute(|(normal, hovered, clicked)| {
            action.render_button(normal, hovered, clicked, rect, window);
            Ok(())
        })?;

        let txt_box = self.begin_btn.render_txt();
        let _ = self.ui_font.execute(|font| {
            // other text
            let txt_other = font
                .render("(or press space)", &FontStyle::new(24., Color::BLACK))
                .unwrap();
            let pos_other = Rectangle::new((270, 375), (125, 24));

            let txt_begin = font
                .render(txt_box.1, &FontStyle::new(24., Color::BLACK))
                .unwrap();
            window.draw(&pos_other, Background::Img(&txt_other));
            window.draw(txt_box.0, Background::Img(&txt_begin));
            Ok(())
        })?;

        let _ = self.ui_image.execute(|img| {
            let pos = Rectangle::new((250, 100), (200, 150));
            window.draw(&pos, Background::Img(img));
            Ok(())
        })?;

        Ok(())
    }
}

impl MainMenu {
    pub fn wants_to_switch(&self) -> Option<LD44States> {
        if self.quit {
            return Some(LD44States::GameState);
        }
        None
    }
}
