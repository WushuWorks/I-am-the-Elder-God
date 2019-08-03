//This is the intro scene, to set up the game

use crate::game_logic::scene_type::SceneReturn;
use crate::game_logic::main_state::{draw_with_center, draw_translate, draw_atlas_with_center};

//Resources
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;

pub struct ElderIntro {
    intro_background: Asset<Image>,
    intro_overlay: Asset<Image>,

    intro_scenes: Asset<Atlas>,
    curr_scene_index: usize,
    max_scenes: usize,

    enter_button: Asset<Image>,
    text: Asset<Image>,
}


impl ElderIntro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let intro_background = "PHGameBackground.png";
        let overlay = "PHOverlayFade.png";
        let enter = "Enter-120x90.png";
        let atlas_index = "Atlas_Intro_Index";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "You are in the intro.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        Ok(Self {
            intro_background: Asset::new(Image::load(intro_background)),
            intro_overlay: Asset::new(Image::load(overlay)),

            intro_scenes: Asset::new(Atlas::load(atlas_index)),
            curr_scene_index: 0,
            max_scenes: 4,

            enter_button: Asset::new(Image::load(enter)),
            text: text_info,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        if window.keyboard()[Key::Return] == Pressed {
            if self.curr_scene_index < self.max_scenes - 1 {
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
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);

        // Draw the background
        draw_with_center(window, &mut self.intro_background, window_center)?;

        // Draws the selected scene with an atlas
        let atlas_key =  ["First", "Second", "Third", "Fourth"]
            .get(self.curr_scene_index)
            .expect("Unhandled scene index in intro::draw");
        draw_atlas_with_center (window, &mut self.intro_scenes, window_center, atlas_key)?;

        // Draw enter button prompt.
        draw_translate (window, &mut self.enter_button, Vector::new(60 + 112, window.screen_size().y as i32 - 180 - 84))?;

        // Draw label text and overlay, label text should always render on top to show the state the game is in
        draw_with_center (window, &mut self.intro_overlay, window_center)?;
        draw_with_center (window, &mut self.text, Vector::new(window_center.x, window_center.y + 286.0))?;

        Ok(())

    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    #[allow(unreachable_patterns, dead_code)]
    pub fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        //Do nothing
        Ok(())
    }
}