//Declare as 'use'able modules
mod game_logic;
mod gameplay_logic;
mod scenes;

use game_logic::main_state::Game;

//Resources
use quicksilver::prelude::*;

fn main() {

    // NOTE: Set HIDPI to 1.0 to get pixel-perfect rendering.
    // Otherwise the window resizes to whatever value the OS sets and
    // scales the contents.
    // https://docs.rs/glutin/0.19.0/glutin/dpi/index.html
    std::env::set_var("WINIT_HIDPI_FACTOR", "1.0");

    let settings = Settings {
        // If the graphics do need to be scaled (e.g. using
        // `with_center`), blur them. This looks better with fonts.

        scale: quicksilver::graphics::ImageScaleStrategy::Blur,
        ..Default::default()
    };
    run::<Game>("I am the Elder God", Vector::new(1024, 768), settings);
}
