use crate::game_logic::scene_type::SceneReturn;

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
        self.intro_background.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window_center.x, window_center.y)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draw the background
        self.intro_overlay.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window_center.x, window_center.y)),
                Img(&image),
            );
            Ok(())
        })?;

        // Draws the selected scene with an atlas
        let atlas_key =  ["First", "Second", "Third", "Fourth"]
            .get(self.curr_scene_index)
            .expect("Unhandled scene index in intro::draw");

        self.intro_scenes.execute(|image| {
            window.draw(
                &image.get(atlas_key).expect("Failed to find key in intro::draw").unwrap_image().area()
                    .with_center((window_center.x, window_center.y)),
                Img(&image.get(atlas_key).expect("Failed to find key in intro::draw").unwrap_image()),
            );
            Ok(())
        })?;

        // Draw enter button prompt.
        self.enter_button.execute(|image| {
            window.draw_ex(
                &image.area()
                    .translate((60 + 112, window.screen_size().y as i32 - 180 - 84)),
                Img(&image),
                Transform::IDENTITY,
                2,
            );
            Ok(())
        })?;

        // Draw label text, This should always render on top to show the state the game is in
        self.text.execute(|image| {
            window.draw_ex(
                &image.area()
                    .with_center((window_center.x, window_center.y + 286.0)),
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
}