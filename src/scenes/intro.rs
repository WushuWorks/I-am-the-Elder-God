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

pub struct ElderIntro {
    intro_background: Asset<Image>,

    intro_scenes: Vec<Asset<Image>>,
    curr_scene_index: usize,
    text: Asset<Image>,
}

impl ElderIntro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let intro_background = "GCSeamlessBackground800x600.png";
        //I declare like this because it is a sensible way to organize arbitrary ordered images
        let intro_scene1 = "FrameSplash1-800x600.png";
        let intro_scene2 = "FrameSplash2-800x600.png";
        let intro_scene3 = "FrameSplash3-800x600.png";
        let intro_scene4 = "FrameSplash4-800x600.png";
        //Declared here so we can get the length below
        let scenes = vec![Asset::new(Image::load(intro_scene1)),
                                            Asset::new(Image::load(intro_scene2)),
                                            Asset::new(Image::load(intro_scene3)),
                                            Asset::new(Image::load(intro_scene4))];

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "Square font am I, intro this is.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));


        Ok(Self {
            intro_background: Asset::new(Image::load(intro_background)),
            intro_scenes: scenes,
            curr_scene_index: 1,
            text: text_info,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        if window.keyboard()[Key::Return] == Pressed {
            if self.curr_scene_index < self.intro_scenes.len() - 1 {
                self.curr_scene_index += 1;
            } else {
                self.curr_scene_index = 0;
                retval = SceneReturn::Finished;
            }

        }

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {

        // Draw the background
        self.intro_background.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw top scene
        self.intro_scenes[self.curr_scene_index].execute(|image| {
            window.draw(
                &image.area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw text
        self.text.execute(|image| {
            window.draw(
                &image.area()
                    .translate((2 + 64, window.screen_size().y as i32 - 30 - 84)),
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
}