use crate::states::game_state::GameState;
use crate::states::main_menu::MainMenu;
use quicksilver::lifecycle::{Event, State, Window};
use quicksilver::prelude::Result;

pub enum LD44States {
    GameState,
    MainMenu,
}

pub struct MainState {
    curr_state: LD44States,
    game_state: GameState,
    main_menu: MainMenu,
}

impl State for MainState {
    fn new() -> Result<Self> {
        let game_state = GameState::new()?;
        let main_menu = MainMenu::new()?;
        let curr_state = LD44States::MainMenu;

        Ok(MainState {
            curr_state,
            game_state,
            main_menu,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        match self.curr_state {
            LD44States::MainMenu => {
                if let Some(new) = self.main_menu.wants_to_switch() {
                    self.curr_state = new;
                }
                self.main_menu.update(window)
            }
            LD44States::GameState => {
                if let Some(new) = self.game_state.wants_to_switch() {
                    self.curr_state = new;
                }
                self.game_state.update(window)
            }
        }
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        match self.curr_state {
            LD44States::MainMenu => self.main_menu.draw(window),
            LD44States::GameState => self.game_state.draw(window),
        }
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match self.curr_state {
            LD44States::MainMenu => self.main_menu.event(event, window),
            LD44States::GameState => self.game_state.event(event, window),
        }
    }
}
