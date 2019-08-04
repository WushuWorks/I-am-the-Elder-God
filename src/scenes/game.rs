use crate::game_logic::scene_type::SceneReturn;
use crate::gameplay_logic::entities::*;
use crate::gameplay_logic::game_board::GameBoard;
use crate::game_logic::main_state::{draw_with_center, draw_atlas_with_center};

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
    cond_tiles: Asset<Atlas>,

    winner: PlayerType,
}

impl ElderGame {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let background = "Fog800x600.png";
        let overlay = "PHOverlayFade.png";
        let atlas_index = "Atlas_Tile_Index";
        let game_atlas_index = "Atlas_Game_Index";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render(
                "WASD - move, 1/2/0 - end",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        //Create players
        let wraith = Entity::new_char(ClassType::Wraith, PlayerType::Player1,
                                      Vector::new(10,12), false)
                                        .expect("Cannot create Wraith game::new.");
        let support = Entity::new_char(ClassType::Support, PlayerType::Player2,
                                       Vector::new(7,5), false)
            .expect("Cannot create Wraith game::new.");
        let assault = Entity::new_char(ClassType::Assault, PlayerType::Player2,
                                       Vector::new(10,4), false)
            .expect("Cannot create Wraith game::new.");
        let trapper = Entity::new_char(ClassType::Trapper, PlayerType::Player2,
                                       Vector::new(13,5), false)
            .expect("Cannot create Wraith game::new.");

        Ok(Self {
            game_background: Asset::new(Image::load(background)),
            game_overlay: Asset::new(Image::load(overlay)),
            text: text_info,

            game_board: GameBoard::new(wraith, support, assault, trapper).expect("Failed to load GameBoard in scenes::game::ElderGame::new"),
            game_tiles: Asset::new(Atlas::load(atlas_index)),
            cond_tiles: Asset::new(Atlas::load(game_atlas_index)),

            winner: PlayerType::Undetermined,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;

        let player: &Entity = self.game_board.get_curr_player()?;
        let info: &PlayerType = player.get_player()?;

        //To make sure only the correct player can issue commands
        match info {
            PlayerType::Player1 => {

            },
            PlayerType::Player2 => {

            },
            PlayerType::Undetermined => {/*no controller, no turn*/}
        }

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
        for row in self.game_board.get_board().unwrap() {
            for cell in row {
                let tile_key = cell.get_land().expect("Failed to get Terrain game::draw").key().expect("No known key for tile.");
                let cond_key = cell.get_cond().expect("Failed to get TerrainStatus game::draw").key().expect("No known key for tile.");
                let pos = cell.get_pos().expect("Failed to get cell position game::draw");

                //Draw land
                draw_atlas_with_center(window, &mut self.game_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 20.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 20.0), tile_key)?;
                //Draw conditions
                draw_atlas_with_center(window, &mut self.cond_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 23.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 18.0), cond_key)?;
            }
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
    pub fn get_winner(&mut self) -> Result<PlayerType> {
        Ok(self.winner)
    }
}