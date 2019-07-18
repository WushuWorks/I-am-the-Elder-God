// I am the Elder God. A 3 vs 1 board game made using Quicksilver
//
// Copyright (C) 2019  WushuWorks
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::game_logic::scene_type::SceneReturn;

//Resources
use quicksilver::prelude::*;


#[allow(unreachable_patterns, dead_code)]
pub struct ElderOutro {
    outro_img: Asset<Image>,
    item_img: Asset<Image>,
    text: Asset<Image>,

    winner: u32,
}

impl ElderOutro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "Square font am I, outro this is.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        //Image Load
        let bob = Asset::new(Image::load("PngBob.png"));
        let game_frame = Asset::new(Image::load("GameFrame800x600.png"));

        Ok(Self {
            outro_img: game_frame,
            item_img: bob,
            text: text_info,

            winner: 0,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        if window.keyboard()[Key::Return] == Pressed {
            retval = SceneReturn::Finished;
        }

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {

        // Draw the frame
        self.outro_img.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw bob
        self.item_img.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw text
        self.text.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((2 + 112, window.screen_size().y as i32 - 30 - 84)),
                Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    #[allow(unreachable_patterns, dead_code)]
    pub fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        //Do nothing
        Ok(())
    }

    #[allow(unreachable_patterns, dead_code)]
    pub fn set_winner(&mut self, winner: &u32) -> Result<()>{
        match winner {

            1 => {
                self.winner = 1;
                Ok(())
            }
            2 => {
                self.winner = 2;
                Ok(())
            }
            _x => {
                panic!("Invalid winner set in ElderOutro::winner")
            }
        }
    }
}