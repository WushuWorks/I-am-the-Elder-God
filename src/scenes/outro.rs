use crate::game_logic::scene_type::{SceneReturn, PlayerType};

//Resource
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;


#[allow(unreachable_patterns, dead_code)]
pub struct ElderOutro {
    outro_background: Asset<Image>,
    outro_scenes: Asset<Atlas>,
    curr_scene_index: usize,
    max_scenes: usize,

    enter_button: Asset<Image>,
    text: Asset<Image>,

    winner: PlayerType,
}

impl ElderOutro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let background = "GCSeamlessBackground800x600.png";
        let enter = "Enter-120x90.png";
        //I declare like this because it is a sensible way to organize arbitrary ordered images
        let atlas_index = "Atlas_Outro_Index";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "Square font am I, outro this is.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        Ok(Self {
            outro_background: Asset::new(Image::load(background)),
            outro_scenes: Asset::new(Atlas::load(atlas_index)),
            curr_scene_index: 0,
            max_scenes: 4,

            enter_button: Asset::new(Image::load(enter)),
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
            if self.curr_scene_index < self.max_scenes - 1 {self.curr_scene_index += 1;}
            else { self.curr_scene_index = 0; retval = SceneReturn::Finished;}
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

        // Draw winners scenes
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

        self.outro_scenes.execute(|image| {
            window.draw(
                &image.get(atlas_key).expect("Failed to find key in outro::draw").unwrap_image().area()
                    .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                Img(&image.get(atlas_key).expect("Failed to find key in outro::draw").unwrap_image()),
            );
            Ok(())
        })?;

        // Draw enter button prompt.
        self.enter_button.execute(|image| {
            window.draw_ex(
                &image.area()
                    .translate((50 + 112, window.screen_size().y as i32 - 150 - 84)),
                Img(&image),
                Transform::IDENTITY,
                2,
            );
            Ok(())
        })?;

        // Draw label text
        // This should always render on top to show the state the game is in
        self.text.execute(|image| {
            window.draw_ex(
                &image.area()
                    .translate((2 + 112, window.screen_size().y as i32 - 30 - 84)),
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

    /// Sets the winner of the game
    /// This can be called during game execution so do not panic! if a 0 is passed
    pub fn set_winner(&mut self, winner: PlayerType) -> Result<()>{
        self.winner = winner;
        Ok(())
    }
}