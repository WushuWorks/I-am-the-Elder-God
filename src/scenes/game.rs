use crate::game_logic::scene_type::SceneReturn;
use crate::gameplay_logic::entities::*;
use crate::gameplay_logic::game_board::GameBoard;
use crate::game_logic::draw_helper::*;

//Resources
use quicksilver::prelude::*;
use quicksilver::graphics::Atlas;
//Std
use std::iter::Cycle;
use std::vec::IntoIter;

#[derive(PartialEq)]
enum ActionType {
    Move,
    Action,
    End,
}

pub struct ElderGame {
    game_background: Asset<Image>,
    game_overlay: Asset<Image>,
    //Label Text
    text: Asset<Image>,
    controls_text: Asset<Image>,
    class_text: Asset<Image>,
    info_text: Asset<Image>,
    abilities_text: Asset<Image>,
    underline: Asset<Image>,
    //White
    move_text: Asset<Image>,
    action_text: Asset<Image>,
    end_text: Asset<Image>,
    //Grey
    move_grey: Asset<Image>,
    action_grey: Asset<Image>,
    end_grey: Asset<Image>,


    //game_board layer
    game_board: GameBoard,

    //Player related data
    player_ref: Vec<Entity>, // players
    turn_order: Cycle<IntoIter<usize>>, // turn index iterator
    curr_player: usize, //index of current player

    //Turn control data - [Move, Action, End]
    end_flag: bool,
    action_state: ActionType,
    max_moves: u32,
    max_actions: u32,

    //Atlas supports keys A-Z, Blank (# is the same tile), and Null (with the '-' key)
    game_tiles: Asset<Atlas>,
    token_tiles: Asset<Atlas>,

    winner: PlayerType,
}

#[allow(unused_assignments)]
impl ElderGame {
    /// Load the assets and initialise the game
    pub fn new() -> Result<Self> {
        let font_mononoki = "square.ttf";
        let background = "Fog800x600.png";
        let overlay = "PHOverlayFade.png";
        let atlas_index = "Atlas_Tile_Index";
        let game_atlas_index = "Atlas_Game_Index";
        let underline = "line.png";

        //Font Load
        let text_info = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Arrow Keys-Move, Q-Action, 0/1/2-end", &FontStyle::new(20.0, Color::BLACK), )}));
        let controls_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[Press]", &FontStyle::new(17.0, Color::WHITE), )}));
        let class_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[Class]", &FontStyle::new(17.0, Color::WHITE), )}));
        let info_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[Stats]", &FontStyle::new(17.0, Color::WHITE), )}));
        let abilities_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[Action]", &FontStyle::new(17.0, Color::WHITE), )}));

        //Menu Words
        let move_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[M]ove", &FontStyle::new(16.0, Color::WHITE), )}));
        let action_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[A]ction", &FontStyle::new(16.0, Color::WHITE), )}));
        let end_text = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[E]nd", &FontStyle::new(16.0, Color::WHITE),)}));
        //Grey Words
        let move_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[M]ove", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let action_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[A]ction", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let end_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("[E]nd", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));

        //Create players
        let wraith = Entity::new_char(ClassType::Wraith, PlayerType::Player1,
                                      Vector::new(9,11), false)
                                        .expect("Cannot create Wraith game::new.");
        let support = Entity::new_char(ClassType::Support, PlayerType::Player2,
                                       Vector::new(6,4), false)
            .expect("Cannot create Support game::new.");
        let assault = Entity::new_char(ClassType::Assault, PlayerType::Player2,
                                       Vector::new(9,3), false)
            .expect("Cannot create Assault game::new.");
        let trapper = Entity::new_char(ClassType::Trapper, PlayerType::Player2,
                                       Vector::new(12,4), false)
            .expect("Cannot create Trapper game::new.");

        //Player Vector
        let player_ref = vec![wraith, support, assault, trapper];
        //Player Turn order, there must be no elements greater than `player_ref.len()-1`
        let mut turn_order = vec![1,0,2,0,3].into_iter().cycle();
        let curr_player = turn_order.next().expect("Cannot find first player");
        //Find first player's stats
        let moves = *player_ref[curr_player].get_stats()?.get_speed();
        let actions = *player_ref[curr_player].get_stats()?.get_actions();

        Ok(Self {
            game_background: Asset::new(Image::load(background)),
            game_overlay: Asset::new(Image::load(overlay)),
            text: text_info,
            controls_text, class_text, info_text, abilities_text,
            underline: Asset::new(Image::load(underline)),
            move_text, action_text, end_text,
            move_grey, action_grey, end_grey,

            game_board: GameBoard::new().expect("Failed to load GameBoard in scenes::game::ElderGame::new"),
            player_ref,
            turn_order,
            curr_player,

            //Turn control data
            end_flag: false,
            action_state: ActionType::Move,
            max_moves: moves,
            max_actions: actions,

            game_tiles: Asset::new(Atlas::load(atlas_index)),
            token_tiles: Asset::new(Atlas::load(game_atlas_index)),

            winner: PlayerType::Undetermined,
        })
    }

    /// Process keyboard and mouse, update the game state
    pub fn update(&mut self, window: &mut Window) -> Result<SceneReturn> {
        use ButtonState::*;
        let mut retval = SceneReturn::Good;
        let kb = window.keyboard();

        let curr_loc = self.player_ref[self.curr_player].get_pos()?;
        let mut moved = false;
        let mut acted = false;

        //Change ActionState - disallow swap if nonsensical
        if kb[Key::M] == Pressed && self.max_moves > 0   {self.action_state = ActionType::Move;}
        if kb[Key::A] == Pressed && self.max_actions > 0 {self.action_state = ActionType::Action;}
        if kb[Key::E] == Pressed                         {self.action_state = ActionType::End;}

        //Only accept commands when the player can do something
        match self.action_state {
            ActionType::Move => {
                if self.max_moves > 0 {
                    if kb[Key::Up] == Pressed { moved = self.try_move(Vector::new(curr_loc.x, curr_loc.y - 1.0))?; }
                    else if kb[Key::Left] == Pressed { moved = self.try_move(Vector::new(curr_loc.x - 1.0, curr_loc.y))?;}
                    else if kb[Key::Down] == Pressed { moved = self.try_move(Vector::new(curr_loc.x, curr_loc.y + 1.0))?;}
                    else if kb[Key::Right] == Pressed { moved = self.try_move(Vector::new(curr_loc.x + 1.0, curr_loc.y))?;}
                }
            },
            ActionType::Action => {
                if self.max_actions > 0 {
                    if kb[Key::Q] == Pressed { acted = true; }
                }
            },
            ActionType::End => {
                self.end_flag = true;
                self.next_turn()?;
            }
        }

        if moved {
            moved = false;
            self.max_moves -= 1;
        }
        if acted {
            acted = false;
            self.max_actions -= 1;
        }

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

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);

        // Draw the frame and overlay
        draw_ex_with_center(window, &mut self.game_background, window_center, Transform::IDENTITY, 1.0)?;
        draw_ex_with_center(window, &mut self.game_overlay, window_center, Transform::IDENTITY, 2.0)?;

        // Draw GameBoard, calculates coordinates from the center for a 19x15 board of 40x40 pixels
        for row in self.game_board.get_board()? {
            for cell in row {
                let tile_key = cell.get_land()?.key().expect("No known key for tile.");;
                let cond_key = cell.get_cond()?.key().expect("No known key for tile.");
                let occupant_key = cell.get_occupant()?.get_class()?.key();
                let pos = cell.get_pos().expect("Failed to get cell position game::draw");

                //Draw land
                draw_ex_atlas_with_center(window, &mut self.game_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 20.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 20.0),
                                          Transform::IDENTITY, 3.0, tile_key)?;
                //Draw occupying objects
                draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 23.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 18.0),
                                       Transform::IDENTITY, 4.0, occupant_key)?;
                //Draw conditions at layer 2
                draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 23.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 18.0),
                                            Transform::IDENTITY, 6.0,cond_key)?;
            }
        }

        //Draw Menus with SmallSquare
        let offsets = vec![Vector::new(1.0, 1.0), Vector::new(1.0, -1.0), Vector::new(-1.0, 1.0), Vector::new(-1.0, -1.0)];
        for offset in offsets {
            draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                      Vector::new(window_center.x - 303.0 * offset.x, window_center.y - 185.0 * offset.y),
                                      Transform::IDENTITY, 6.0, "SmallSquare")?;
        }
        //Draw Selected Player Class Label
        let curr_class_key = self.player_ref[self.curr_player].get_class()?.key().to_owned() + "Class";
        draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                  Vector::new(window_center.x - 303.0, window_center.y - 171.0),
                                  Transform::IDENTITY, 6.1, &curr_class_key[..])?;

        //Draw Players
        for player in self.player_ref.iter() {
            let player_pos = player.get_pos()?;
            let player_key = player.get_class()?.key();
            draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                   Vector::new(window_center.x - 380.0 + (40.0 * player_pos.x) + 23.0,
                                               window_center.y - 300.0 + (40.0 * player_pos.y) + 18.0), Transform::IDENTITY, 5.0, player_key)?;
        }

        // Draw Menu Labels
        draw_ex_with_center(window, &mut self.controls_text, Vector::new(window_center.x - 303.0, window_center.y + 135.0),
                            Transform::IDENTITY, 8.01)?;
        draw_ex_with_center(window, &mut self.class_text, Vector::new(window_center.x - 303.0, window_center.y - 235.0),
                            Transform::IDENTITY, 8.02)?;
        draw_ex_with_center(window, &mut self.info_text, Vector::new(window_center.x + 303.0, window_center.y - 235.0),
                            Transform::IDENTITY, 8.03)?;
        draw_ex_with_center(window, &mut self.abilities_text, Vector::new(window_center.x + 303.0, window_center.y + 135.0),
                            Transform::IDENTITY, 8.04)?;

        // Draw State Indicator
        match self.action_state {
            ActionType::Move => {
                draw_ex_with_center(window, &mut self.underline, Vector::new(window_center.x - 301.0, window_center.y + 186.0),
                                    Transform::IDENTITY, 8.05)?; },
            ActionType::Action => {
                draw_ex_with_center(window, &mut self.underline, Vector::new(window_center.x - 301.0, window_center.y + 216.0),
                                    Transform::IDENTITY, 8.05)?; },
            ActionType::End => {
                draw_ex_with_center(window, &mut self.underline, Vector::new(window_center.x - 301.0, window_center.y + 246.0),
                                    Transform::IDENTITY, 8.05)?; }
        }

        // Draw label text items, should always render on top to show the state the game is in
        draw_ex_with_center(window, &mut self.move_grey, Vector::new(window_center.x - 303.0, window_center.y + 175.0),
                            Transform::IDENTITY, 8.11)?;
        draw_ex_with_center(window, &mut self.action_grey, Vector::new(window_center.x - 303.0, window_center.y + 205.0),
                            Transform::IDENTITY, 8.12)?;
        draw_ex_with_center(window, &mut self.end_grey, Vector::new(window_center.x - 303.0, window_center.y + 235.0),
                            Transform::IDENTITY, 8.13)?;

        //Render the white button if the player can do the action
        if self.max_moves > 0 {
            draw_ex_with_center(window, &mut self.move_text, Vector::new(window_center.x - 303.0, window_center.y + 175.0),
                                Transform::IDENTITY, 8.21)?; }
        if self.max_actions > 0 {
            draw_ex_with_center(window, &mut self.action_text, Vector::new(window_center.x - 303.0, window_center.y + 205.0),
                                Transform::IDENTITY, 8.22)?; }
        if !self.end_flag {
            draw_ex_with_center(window, &mut self.end_text, Vector::new(window_center.x - 303.0, window_center.y + 235.0),
                                Transform::IDENTITY, 8.23)?; }

        draw_ex_with_center(window, &mut self.text, Vector::new(window_center.x, window_center.y + 286.0),
        Transform::IDENTITY, 8.4)?;

        Ok(())
    }

    /// Handle various sorts of events, https://docs.rs/quicksilver/0.3.16/quicksilver/lifecycle/enum.Event.html
    pub fn event(&mut self, _event: &Event, _window: &mut Window) -> Result<()> {
        Ok(())
    }

    /// Special function that decides who is the winner of the game
    /// This should only be defined in a scene where a winner is relevant info, like a game
    pub fn get_winner(&mut self) -> Result<PlayerType> {
        Ok(self.winner)
    }

    ///Shifts index to the next player's turn, and sets variables
    pub fn next_turn(&mut self) -> Result<()> {
        self.curr_player = self.turn_order.next().expect("Cannot find next player index game::next_turn");
        self.max_moves = *self.player_ref[self.curr_player].get_stats()?.get_speed();
        self.max_actions = *self.player_ref[self.curr_player].get_stats()?.get_actions();
        self.action_state = ActionType::Move;
        self.end_flag = false;

        Ok(())
    }

    ///Tries to move a player, returns true if moved, false otherwise
    pub fn try_move(&mut self, new_loc: Vector) -> Result<bool> {
        let mut retval= false;

        if self.player_ref[self.curr_player].can_move(new_loc, &self.game_board, &self.player_ref)? {
            self.player_ref[self.curr_player].set_pos(new_loc)?;
            retval = true;
        }

        Ok(retval)
    }

}