/*
Here we define the overarching 'Game' which contains all of its sub-components and is the core loop.
*/

use crate::game_logic::scene_type::{SceneType, SceneReturn};
use crate::game_logic::music_player::MusicPlayer;
use crate::game_logic::draw_helper::*;
use crate::scenes::game::ElderGame;
use crate::scenes::intro::ElderIntro;
use crate::scenes::outro::ElderOutro;

//Resources
use quicksilver::prelude::*;
//Std imports
use std::vec::IntoIter;
use std::iter::Cycle;


pub struct Game {
    //For scene order control
    curr_scene: SceneType,
    scene_circle_iterator: Cycle<IntoIter<SceneType>>,

    //Scene Data
    intro_scenes: ElderIntro,
    game_scenes: ElderGame,
    outro_scenes: ElderOutro,

    //Large Files
    overlay: Asset<Image>,
    bg_music: MusicPlayer,
}

impl State for Game {
    /// Load the assets and initialise the game
    fn new() -> Result<Self> {

        //Scene resource allocations, this defines states
        let intro = ElderIntro::new().expect("Cannot load Elder Intro");
        let game = ElderGame::new().expect("Cannot load Elder Game");
        let outro = ElderOutro::new().expect("Cannot load Elder Outro");

        //Large/universal data allocations, waste not want not
        let music = MusicPlayer::new("vgm21.wav", 19.0, 1.0)
            .expect("Cannot initialize MusicPlayer in main_state::new");
        let game_overlay = Asset::new(Image::load("PHOverlay.png"));

        //Scene order allocation, this defines the order of states
        let scenes: Vec<SceneType> = vec![SceneType::Intro, SceneType::Game, SceneType::Outro];
        let mut scene_cycle: Cycle<IntoIter<SceneType>> = scenes.into_iter().cycle();
        let first_scene: SceneType = scene_cycle.next().expect("Empty scene buffer in Game::new(), cannot continue.");


        Ok(Self {
            curr_scene: first_scene,
            scene_circle_iterator: scene_cycle,

            intro_scenes: intro,
            game_scenes: game,
            outro_scenes: outro,

            //Large Files
            overlay: game_overlay,
            bg_music: music,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        let scene_flag = match self.curr_scene {
            SceneType::Intro => self.intro_scenes.update(window)?,
            SceneType::Game => {
                let scene_retval = self.game_scenes.update(window)?;
                self.outro_scenes.set_winner(self.game_scenes.get_winner()?)?;
                scene_retval
            }
            SceneType::Outro => {
                self.outro_scenes.update(window)?
            }
        };

        match scene_flag {
            SceneReturn::Good => Ok(()), //Do not transition
            SceneReturn::Finished => { //Do transition
                self.curr_scene = self.scene_circle_iterator.next().unwrap();
                Ok(())
            }
        }
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        //Do nothing for now
        Ok(())
    }

    /// Draw stuff on the screen
    /// Note that since the center gap is 800x600 you can offset coordinates to compensate for this
    /// by keeping in mind that there is a 112p horizonatal gap and 84p vertical gap on from a single side
    ///
    /// Correction Offset Example - here we render from the center frame's bottom left corner
    /// ```translate((2 + 112, window.screen_size().y as i32 - 30 - 84))```
    ///
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);
        //See music_player.rs for reasoning
        self.bg_music.play_if_not(window.current_fps())?;

        //Draw overlay first to put it on the bottom.
        draw_with_center(window, &mut self.overlay, window_center)?;

        //Result is passed up
        let retval = match self.curr_scene {
            SceneType::Intro => self.intro_scenes.draw(window),
            SceneType::Game => self.game_scenes.draw(window),
            SceneType::Outro => self.outro_scenes.draw(window),
        };

        retval
    }
}