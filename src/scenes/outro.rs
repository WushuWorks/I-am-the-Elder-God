use crate::game_logic::scene_type::{SceneReturn, PlayerType};

//Resource
use quicksilver::prelude::*;


#[allow(unreachable_patterns, dead_code)]
pub struct ElderOutro {
    outro_background: Asset<Image>,
    outro_scenes_p0: Vec<Asset<Image>>,
    outro_scenes_p1: Vec<Asset<Image>>,
    outro_scenes_p2: Vec<Asset<Image>>,
    curr_scene_index_p0: usize,
    curr_scene_index_p1: usize,
    curr_scene_index_p2: usize,

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
        //P0 Victory scenes
        let p0_scene = ("P0Win1-800x600.png", "P0Win2-800x600.png", "P0Win3-800x600.png", "P0Win4-800x600.png");
        //P1 Victory scenes
        let p1_scene = ("P1Win1-800x600.png", "P1Win2-800x600.png", "P1Win3-800x600.png", "P1Win4-800x600.png");
        //P2 victory scenes
        let p2_scene = ("P2Win1-800x600.png", "P2Win2-800x600.png", "P2Win3-800x600.png", "P2Win4-800x600.png");
        //P0 scene vec
        let p0_scenes = vec![Asset::new(Image::load(p0_scene.0)),
                                                Asset::new(Image::load(p0_scene.1)),
                                                Asset::new(Image::load(p0_scene.2)),
                                                Asset::new(Image::load(p0_scene.3)),];
        //P1 scene vec
        let p1_scenes = vec![Asset::new(Image::load(p1_scene.0)),
                                                Asset::new(Image::load(p1_scene.1)),
                                                Asset::new(Image::load(p1_scene.2)),
                                                Asset::new(Image::load(p1_scene.3)),];
        //P2 scene vec
        let p2_scenes = vec![Asset::new(Image::load(p2_scene.0)),
                                                Asset::new(Image::load(p2_scene.1)),
                                                Asset::new(Image::load(p2_scene.2)),
                                                Asset::new(Image::load(p2_scene.3)),];

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "Square font am I, outro this is.",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        Ok(Self {
            outro_background: Asset::new(Image::load(background)),
            outro_scenes_p0: p0_scenes,
            outro_scenes_p1: p1_scenes,
            outro_scenes_p2: p2_scenes,
            curr_scene_index_p0: 0,
            curr_scene_index_p1: 0,
            curr_scene_index_p2: 0,

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
            match self.winner {
                PlayerType::Undetermined => {
                    if self.curr_scene_index_p0 < self.outro_scenes_p0.len() - 1 {self.curr_scene_index_p0 += 1;}
                    else { self.curr_scene_index_p0 = 0; retval = SceneReturn::Finished;}
                },
                PlayerType::Player1 => {
                    if self.curr_scene_index_p1 < self.outro_scenes_p1.len() - 1 {self.curr_scene_index_p1 += 1;}
                    else { self.curr_scene_index_p1 = 0; retval = SceneReturn::Finished;}
                },
                PlayerType::Player2 => {
                    if self.curr_scene_index_p2 < self.outro_scenes_p2.len() - 1 {self.curr_scene_index_p2 += 1;}
                    else { self.curr_scene_index_p2 = 0; retval = SceneReturn::Finished;}
                },
            }
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
        match self.winner {
            PlayerType::Undetermined => {
                self.outro_scenes_p0[self.curr_scene_index_p0].execute(|image| {
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
            PlayerType::Player1 => {
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
            PlayerType::Player2 => {
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
        }

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