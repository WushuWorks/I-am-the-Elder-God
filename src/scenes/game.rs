use crate::game_logic::scene_type::SceneReturn;
use crate::gameplay_logic::entities::*;
use crate::gameplay_logic::game_board::GameBoard;
use crate::game_logic::draw_helper::*;

//Resources
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;

pub struct ElderGame {
    game_background: Asset<Image>,
    game_overlay: Asset<Image>,
    text: Asset<Image>,

    //game_board layer
    game_board: GameBoard,

    //Player related data
    player_ref: Vec<Entity>, // players
    curr_player: usize, //index of current player


    //Atlas supports keys A-Z, Blank (# is the same tile), and Null (with the '-' key)
    game_tiles: Asset<Atlas>,
    token_tiles: Asset<Atlas>,

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
                "WASD-Move,[Z,X,C,V]-Swap, 0/1/2-end",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));

        //Create players
        let wraith = Entity::new_char(ClassType::Wraith, PlayerType::Player1,
                                      Vector::new(9,11), false)
                                        .expect("Cannot create Wraith game::new.");
        let support = Entity::new_char(ClassType::Support, PlayerType::Player2,
                                       Vector::new(6,4), false)
            .expect("Cannot create Wraith game::new.");
        let assault = Entity::new_char(ClassType::Assault, PlayerType::Player2,
                                       Vector::new(9,3), false)
            .expect("Cannot create Wraith game::new.");
        let trapper = Entity::new_char(ClassType::Trapper, PlayerType::Player2,
                                       Vector::new(12,4), false)
            .expect("Cannot create Wraith game::new.");
        let player_ref = vec![wraith, support, assault, trapper];


        Ok(Self {
            game_background: Asset::new(Image::load(background)),
            game_overlay: Asset::new(Image::load(overlay)),
            text: text_info,

            game_board: GameBoard::new().expect("Failed to load GameBoard in scenes::game::ElderGame::new"),
            player_ref,
            curr_player: 0,

            game_tiles: Asset::new(Atlas::load(atlas_index)),
            token_tiles: Asset::new(Atlas::load(game_atlas_index)),

            winner: PlayerType::Undetermined,
        })
    }

    /// Process keyboard and mouse, update the game state
    /// Handles end game checking
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;
        let kb = window.keyboard();

        if kb[Key::Key0] == Pressed {
            self.winner = PlayerType::Undetermined;
            retval = SceneReturn::Finished;
        }
        if kb[Key::Key1] == Pressed {
            self.winner = PlayerType::Player1;
            retval = SceneReturn::Finished;
        }
        if kb[Key::Key2] == Pressed {
            self.winner = PlayerType::Player2;
            retval = SceneReturn::Finished;
        }
        if kb[Key::Z] == Pressed {self.set_turn(0)?;}
        if kb[Key::X] == Pressed {self.set_turn(1)?;}
        if kb[Key::C] == Pressed {self.set_turn(2)?;}
        if kb[Key::V] == Pressed {self.set_turn(3)?;}

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);

        // Draw the frame and overlay
        draw_with_center(window, &mut self.game_background, window_center)?;
        draw_with_center(window, &mut self.game_overlay, window_center)?;

        // Draw GameBoard, calculates coordinates from the center for a 19x15 board of 40x40 pixels
        for row in self.game_board.get_board()? {
            for cell in row {
                let tile_key = cell.get_land()?.key().expect("No known key for tile.");;
                let cond_key = cell.get_cond()?.key().expect("No known key for tile.");
                let occupant_key = cell.get_occupant()?.get_class()?.key();
                let pos = cell.get_pos().expect("Failed to get cell position game::draw");

                //Draw land
                draw_atlas_with_center(window, &mut self.game_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 20.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 20.0), tile_key)?;
                //Draw occupying objects
                draw_atlas_with_center(window, &mut self.token_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 23.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 18.0), occupant_key)?;
                //Draw conditions at layer 2
                draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 23.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 18.0),
                                            Transform::IDENTITY, 2.0,cond_key)?;
            }
        }

        //Draw Players
        for player in self.player_ref.iter() {
            let player_pos = player.get_pos()?;
            let player_key = player.get_class()?.key();
            draw_atlas_with_center(window, &mut self.token_tiles,
                                   Vector::new(window_center.x - 380.0 + (40.0 * player_pos.x) + 23.0,
                                               window_center.y - 300.0 + (40.0 * player_pos.y) + 18.0), player_key)?;
        }

        // Draw label text, should always render on top to show the state the game is in
        draw_ex_with_center(window, &mut self.text, Vector::new(window_center.x, window_center.y + 286.0),
        Transform::IDENTITY, 3.0)?;

        Ok(())
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    #[allow(unreachable_patterns, dead_code)]
    pub fn event(&mut self, event: &Event, _window: &mut Window) -> Result<()> {
        use ButtonState::*;

        let player: Entity = self.player_ref[self.curr_player];
        let curr_loc = player.get_pos()?;

        match event {
            Event::Key(k, bs) => {
                if *bs == Pressed {
                    let new_loc = match k {
                            Key::W => { Vector::new(curr_loc.x, curr_loc.y - 1.0)},
                            Key::S => { Vector::new(curr_loc.x, curr_loc.y + 1.0)},
                            Key::A => { Vector::new(curr_loc.x - 1.0, curr_loc.y)},
                            Key::D => { Vector::new(curr_loc.x + 1.0, curr_loc.y)},
                            _       => { curr_loc },
                    };

                    if self.player_ref[self.curr_player].can_move(new_loc, &self.game_board, &self.player_ref)? {
                        self.player_ref[self.curr_player].set_pos(new_loc)?;
                        //self.next_turn()?;
                    } else {
                        //println!("Can't Move!");
                    }
                }
            },
            _ => {},
        }

        Ok(())
    }

    /// Special function that decides who is the winner of the game
    /// This should only be defined in a scene where a winner is relevant info, like a game
    pub fn get_winner(&mut self) -> Result<PlayerType> {
        Ok(self.winner)
    }


    ///Shifts index to the next player's turn
    pub fn next_turn(&mut self) -> Result<()> {
        if self.curr_player as usize >= self.player_ref.len() - 1 { //Time to reset
            self.curr_player = 0;
        } else {
            self.curr_player += 1;
        }
        Ok(())
    }

    ///Shifts index to the selected players turn
    pub fn set_turn(&mut self, player_index: usize) -> Result<bool> {
        let mut set_player = true; //Assume truth and disprove if needed

        if player_index <= self.player_ref.len() - 1 {
            self.curr_player = player_index;
        } else {
            set_player = false;
        }

        Ok(set_player)
    }

}