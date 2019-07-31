use crate::game_logic::scene_type::SceneReturn;

//Resources
use quicksilver::prelude::*;

pub struct ElderIntro {
    intro_background: Asset<Image>,
    intro_scenes: Vec<Asset<Image>>,
    curr_scene_index: usize,

    enter_button: Asset<Image>,
    text: Asset<Image>,
}


impl ElderIntro {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let intro_background = "GCSeamlessBackground800x600.png";
        let enter = "Enter-120x90.png";
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
            curr_scene_index: 0,

            enter_button: Asset::new(Image::load(enter)),
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

        // Draws the selected scene note that we draw scenes here and explicitly set the z coordinate
        //
        // ```window.draw_ex(draw: &Drawable, bkg: Into<Background<'a>>, trans: Transform, z: Scalar```
        self.intro_scenes[self.curr_scene_index].execute(|image| {
             window.draw_ex(
                    &image.area()
                        .with_center((window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2)),
                    Img(&image),
                 Transform::IDENTITY,
                 1,
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
}