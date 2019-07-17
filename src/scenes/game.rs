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

use game_logic::scene_type::SceneReturn;

struct ElderGame {
    game_img: Asset<Image>,
    item_img: Asset<Image>,
    text: Asset<Image>,
    sound: Asset<Sound>,
    winner: u32,
}

impl ElderGame {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let music = "vgm21.wav";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "Square font am I, game this is.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }))?;

        //Music Load
        let music = Asset::new( Sound::load(music));

        //Image Load
        let bob = Asset::new(Image::load("PngBob.png"))?;
        let game_frame = Asset::new(Image::load("GameFrame.png"))?;

        Ok(Self {
            game_img: game_frame,
            item_img: bob,
            text: text_info,
            sound: music,
            winner: 0,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        if window.keyboard()[Key::Return] == Pressed {
            self.sound.execute(|music| {music.play()})?;
            retval = Ok(SceneReturn::Finished);
        }

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {

        // Draw the frame
        self.game_img.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw bob
        self.item_img.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw text
        self.text.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate((2, window.screen_size().y as i32 - 30)),
                Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    pub fn event(&mut self, window: &mut Window) -> Result<()> {
        //Do nothing
        Ok(())
    }

    /// Special function that decides who is the winner of the game
    /// This should only be defined in a scene where a winner is relevant info, like a game
    pub fn get_winner(&mut self)  -> Result<()>{
        self.winner = 1337;
        Ok(winner)
    }
}