use crate::game_logic::scene_type::{SceneReturn, PlayerType};
use crate::gameplay_logic::game_board::GameBoard;
use crate::game_logic::main_state::{draw_with_center, draw_translate, draw_atlas_with_center};

//Resources
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;

pub struct ElderGame {
    game_background: Asset<Image>,
    game_overlay: Asset<Image>,
    text: Asset<Image>,

    //game_board layer
    game_board: GameBoard,
    //Atlas supports keys A-Z, Blank (# is the same tile), and Null (with the '-' key)
    game_tiles: Asset<Atlas>,

    winner: PlayerType,
}

impl ElderGame {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let background = "Fog800x600.png";
        let overlay = "PHOverlayFade.png";
        let atlas_index = "Atlas_Tile_Index";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then( |font| {
            font.render(
                "WASD - move, 1/2/0 - end",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        Ok(Self {
            game_background: Asset::new(Image::load(background)),
            game_overlay: Asset::new(Image::load(overlay)),
            text: text_info,

            game_board: GameBoard::new().expect("Failed to load GameBoard in scenes::game::ElderGame::new"),
            game_tiles: Asset::new(Atlas::load(atlas_index)),

            winner: PlayerType::Undetermined,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        if window.keyboard()[Key::Key0] == Pressed {
            self.winner = PlayerType::Undetermined;
            retval = SceneReturn::Finished;
        }
        if window.keyboard()[Key::Key1] == Pressed {
            self.winner = PlayerType::Player1;
            retval = SceneReturn::Finished;
        }
        if window.keyboard()[Key::Key2] == Pressed {
            self.winner = PlayerType::Player2;
            retval = SceneReturn::Finished;
        }

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);

        // Draw the frame and overlay
        draw_with_center(window, &mut self.game_background, window_center)?;
        draw_with_center(window, &mut self.game_overlay, window_center)?;

        // Draw GameBoard, calculates coordinates from the center for a 19x15 board of 40x40 pixels
        for cell in self.game_board.get_board().unwrap() {
            let tile_key = cell.get_land().expect("Failed to get Terrain game::draw").key().expect("No known key for tile.");
            let pos = cell.get_pos().expect("Failed to get cell position game::draw");

            draw_atlas_with_center(window, &mut self.game_tiles,
                                   Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 20.0,
                                                         window_center.y - 300.0 + (40.0 * pos.y) + 20.0), tile_key)?;
        }

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

    /// Special function that decides who is the winner of the game
    /// This should only be defined in a scene where a winner is relevant info, like a game
    pub fn get_winner(&mut self)  -> Result<PlayerType>{
        Ok(self.winner)
    }
}