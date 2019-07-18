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

/*
Here we define the overarching 'Game' which contains all of its sub-components and is the core loop
*/

use crate::game_logic::scene_type::{SceneType, SceneReturn};
use crate::scenes::game::ElderGame;
use crate::scenes::intro::ElderIntro;
use crate::scenes::outro::ElderOutro;

//Resources
use quicksilver::prelude::*;
//Std imports
use std::slice::Iter;
use std::iter::Cycle;

pub struct Game<'a> {
    //For scene order control
    curr_scene: &'a SceneType,
    scene_circle_iterator: Cycle<Iter<'a,SceneType>>,
    scene_vector: Vec<SceneType>,

    //Scene Data
    intro_scenes: ElderIntro,
    game_scenes: ElderGame,
    outro_scenes: ElderOutro,

    //Game Winner
    winner: u32,
}

impl<'a> State for Game<'a> {
    /// Load the assets and initialise the game
    fn new() -> Result<Self> {

        //Scene resource allocations, this defines states
        let intro = ElderIntro::new().expect("Cannot load Elder Intro");
        let game = ElderGame::new().expect("Cannot load Elder Game");
        let outro = ElderOutro::new().expect("Cannot load Elder Outro");

        //Scene order allocation, this defines the order of states
        let mut scenes: Vec<SceneType> = vec![SceneType::Intro, SceneType::Game, SceneType::Outro];
        let mut scene_cycle: Cycle<Iter<SceneType>> = scenes.iter().cycle();
        let first_scene: &SceneType = scene_cycle.next().expect("Empty scene buffer in Game::new(), cannot continue.");


        Ok(Self {
            curr_scene: first_scene,
            scene_circle_iterator: scene_cycle,
            scene_vector: scenes,

            intro_scenes: intro,
            game_scenes: game,
            outro_scenes: outro,

            winner: 0,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {

        let scene_flag = match self.curr_scene {
            SceneType::Intro     => self.intro_scenes.update(window)?,
            SceneType::Game      => {
                let scene_retval = self.game_scenes.update(window)?;
                self.winner = self.game_scenes.get_winner()?;
                scene_retval
            },
            SceneType::Outro     => {
                self.outro_scenes.update(window)?
            },
            _                    => panic!("Unhandled scene type {:?} encountered in MainState update.", self.curr_scene),
        };

        match scene_flag {
            SceneReturn::Good => Ok(()), //Do not transition
            SceneReturn::Finished => { //Do transition
                self.curr_scene = self.scene_circle_iterator.next().unwrap();
                Ok(())
            },
            _x => panic!("Error in MainState key_down_event call: {:?}", _x),
        }
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        //Do nothing for now
        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {

        let msg = match self.curr_scene {
            SceneType::Intro     => self.intro_scenes.draw(window)?,
            SceneType::Game      => self.game_scenes.draw(window)?,
            SceneType::Outro     => self.outro_scenes.draw(window)?,
            _                    => panic!("Unhandled scene type {:?} encountered in MainState draw.", self.curr_scene),
        };
    }



}