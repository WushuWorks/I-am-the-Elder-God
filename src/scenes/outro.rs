use crate::game_logic::scene_type::SceneReturn;
use crate::gameplay_logic::entities::PlayerType;
use crate::game_logic::draw_helper::*;

//Resource
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;

#[allow(unreachable_patterns, dead_code)]
pub struct ElderOutro {
    outro_background: Asset<Image>,
    outro_overlay: Asset<Image>,

    outro_scenes: Asset<Atlas>,
    curr_scene_index: usize,
    max_scenes: usize,

    text: Asset<Image>,

    winner: PlayerType,
}

impl ElderOutro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let background = "PHGameBackground.png";
        let overlay = "PHOverlayFade.png";
        //I declare like this because it is a sensible way to organize arbitrary ordered images
        let atlas_index = "Atlas_Outro_Index";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "Game set match. [Enter] to progress",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        Ok(Self {
            outro_background: Asset::new(Image::load(background)),
            outro_overlay: Asset::new(Image::load(overlay)),

            outro_scenes: Asset::new(Atlas::load(atlas_index)),
            curr_scene_index: 0,
            max_scenes: 4,

            text: text_info,

            winner: PlayerType::Undetermined,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        if window.keyboard()[Key::Return] == Pressed {
            // Matches the winner and increments their scene counters.
            // Resetting and finishing when done
            if self.curr_scene_index < self.max_scenes - 1 { self.curr_scene_index += 1; } else {
                self.curr_scene_index = 0;
                retval = SceneReturn::Finished;
            }
        }

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);

        // Draw the frame and overlay
        draw_with_center(window, &mut self.outro_background, window_center)?;
        draw_with_center(window, &mut self.outro_overlay, window_center)?;

        // Draw winner's scenes
        let atlas_key = *match self.winner {
            PlayerType::Undetermined => ["P0_First", "P0_Second", "P0_Third", "P0_Fourth"]
                .get(self.curr_scene_index)
                .expect("Unhandled scene index in P0 outro::draw"),
            PlayerType::Player1 => ["P1_First", "P1_Second", "P1_Third", "P1_Fourth"]
                .get(self.curr_scene_index)
                .expect("Unhandled scene index in P1 outro::draw"),
            PlayerType::Player2 => ["P2_First", "P2_Second", "P2_Third", "P2_Fourth"]
                .get(self.curr_scene_index)
                .expect("Unhandled scene index in P2 outro::draw"),
        };
        draw_atlas_with_center(window, &mut self.outro_scenes, window_center, atlas_key)?;

        // Draw label text, should always render on top to show the state the game is in
        draw_with_center(window, &mut self.text, Vector::new(window_center.x, window_center.y + 286.0))?;

        Ok(())
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    #[allow(unreachable_patterns, dead_code)]
    pub fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        //Do nothing
        Ok(())
    }

    /// Sets the winner of the game
    /// This can be called during game execution so do not panic! if a 0 is passed
    pub fn set_winner(&mut self, winner: PlayerType) -> Result<()> {
        self.winner = winner;
        Ok(())
    }
}