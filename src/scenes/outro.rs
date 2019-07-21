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
    outro_background: Asset<Image>,
    outro_scenes_p1: Vec<Asset<Image>>,
    outro_scenes_p2: Vec<Asset<Image>>,
    curr_scene_index_p1: usize,
    curr_scene_index_p2: usize,
    text: Asset<Image>,

    winner: u32,
}

impl ElderOutro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let background = "GCSeamlessBackground800x600.png";
        //I declare like this because it is a sensible way to organize arbitrary ordered images
        //P1 Victory scenes
        let p1_scene1 = "P1Win1800x600.png";
        let p1_scene2 = "P1Win2800x600.png";
        let p1_scene3 = "P1Win3800x600.png";
        let p1_scene4 = "P1Win4800x600.png";
        //P2 victory scenes
        let p2_scene1 = "P2Win1800x600.png";
        let p2_scene2 = "P2Win2800x600.png";
        let p2_scene3 = "P2Win3800x600.png";
        let p2_scene4 = "P2Win4800x600.png";
        //P1 scene vec
        let p1_scenes = vec![Asset::new(Image::load(p1_scene1)),
                                            Asset::new(Image::load(p1_scene2)),
                                            Asset::new(Image::load(p1_scene3)),
                                            Asset::new(Image::load(p1_scene4)),];
        //P2 scene vec
        let p2_scenes = vec![Asset::new(Image::load(p2_scene1)),
                                            Asset::new(Image::load(p2_scene2)),
                                            Asset::new(Image::load(p2_scene3)),
                                            Asset::new(Image::load(p2_scene4)),];

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "Square font am I, outro this is.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        //Image Load
        let game_frame = Asset::new(Image::load("GameFrame800x600.png"));

        Ok(Self {
            outro_background: Asset::new(Image::load(background)),
            outro_scenes_p1: p1_scenes,
            outro_scenes_p2: p2_scenes,
            curr_scene_index_p1: 0,
            curr_scene_index_p2: 0,
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
        self.outro_background.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                Img(&image),
            );
            Ok(())
        })?;

        //Draw winners scenes
        match self.winner {
            1 => {
                self.outro_scenes_p1[self.curr_scene_index_p1].execute(|image| {
                    window.draw_ex(
                        &image.area()
                            .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                        Img(&image),
                        Transform::IDENTITY,
                        1,
                    );
                    Ok(())
                })?;
            },
            2 => {
                self.outro_scenes_p2[self.curr_scene_index_p2].execute(|image| {
                    window.draw_ex(
                        &image.area()
                            .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                        Img(&image),
                        Transform::IDENTITY,
                        1,
                    );
                    Ok(())
                })?;
            },
            _e => { if _e == 0{panic!("Unhandled player winner: {:?}, encountered in outro::draw, perhaps it was not set correctly?", _e)}
                    else { panic!("Unhandled player winner: {:?}, encountered in outro::draw", _e) }
                    }
        }

        /// Draw label text
        /// This should always render on top to show the state the game is in
        self.text.execute(|image| {
            window.draw_ex(
                &image.area()
                    .translate((2 + 64, window.screen_size().y as i32 - 30 - 84)),
                Img(&image),
                Transform::IDENTITY,
                2,
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