mod components;
mod ld_entities;
mod level;
mod renderer;
mod resources;
mod states;
mod stuff;
mod systems;
mod ui;

use quicksilver::geom::Vector;
use quicksilver::lifecycle::{run, Settings};
use states::main_state::MainState;

fn main() {
    run::<MainState>("LD44", Vector::new(640, 480), Settings::default());
}
