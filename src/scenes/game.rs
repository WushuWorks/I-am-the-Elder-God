use crate::game_logic::scene_type::SceneReturn;
use crate::gameplay_logic::entities::*;
use crate::gameplay_logic::animator::Animator;
use crate::gameplay_logic::game_board::GameBoard;
use crate::game_logic::draw_helper::*;

//Resources
use quicksilver::prelude::*;
use quicksilver::graphics::{Atlas};
//Std
use std::iter::Cycle;
use std::vec::IntoIter;

#[derive(PartialEq)]
enum ActionType {
    Move,
    Action,
    End,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
}

pub struct ElderGame {
    game_background: Asset<Image>,
    game_overlay: Asset<Image>,

    //Text
    //Help
    move_help: Asset<Image>,
    action_help: Asset<Image>,
    end_help: Asset<Image>,
    //Menu Labels
    controls_text: Asset<Image>,
    class_text: Asset<Image>,
    info_text: Asset<Image>,
    abilities_text: Asset<Image>,
    underline: Asset<Image>,
    //White
    move_text: Asset<Image>, action_text: Asset<Image>, end_text: Asset<Image>,
    //Grey
    move_grey: Asset<Image>, action_grey: Asset<Image>, end_grey: Asset<Image>,

    //Stat Labels
    hp_label: Asset<Image>,
    move_label: Asset<Image>,
    team_label: Asset<Image>,
    sat_label: Asset<Image>, elder_label: Asset<Image>,
    //Class Ability Labels
    //Wraith
    drain_grey: Asset<Image>, decoy_grey: Asset<Image>, rend_grey: Asset<Image>,
    drain_white: Asset<Image>, decoy_white: Asset<Image>, rend_white: Asset<Image>,
    drain_help: Asset<Image>, decoy_help: Asset<Image>, rend_help: Asset<Image>,
    //Support
    bio_grey: Asset<Image>, shield_grey: Asset<Image>, renew_grey: Asset<Image>,
    bio_white: Asset<Image>, shield_white: Asset<Image>, renew_white: Asset<Image>,
    bio_help: Asset<Image>, shield_help: Asset<Image>, renew_help: Asset<Image>,
    //Assault
    pierce_grey: Asset<Image>, grenade_grey: Asset<Image>, airraid_grey: Asset<Image>,
    pierce_white: Asset<Image>, grenade_white: Asset<Image>, airraid_white: Asset<Image>,
    pierce_help: Asset<Image>, grenade_help: Asset<Image>, airraid_help: Asset<Image>,
    //Trapper
    caltrop_grey: Asset<Image>, spear_grey: Asset<Image>, cage_grey: Asset<Image>,
    caltrop_white: Asset<Image>, spear_white: Asset<Image>, cage_white: Asset<Image>,
    caltrop_help: Asset<Image>, spear_help: Asset<Image>, cage_help: Asset<Image>,

    //game_board layer
    game_board: GameBoard,

    //Player related data
    player_ref: Vec<Entity>, // players
    turn_order: Cycle<IntoIter<usize>>, // turn index iterator
    curr_player: usize, //index of current player

    //Turn control data - [Move, Action, End]
    end_flag: bool,
    action_state: ActionType,
    moves: u32,
    actions: u32,
    curr_dir: Direction,
    directions: Cycle<IntoIter<Direction>>,

    selections: Cycle<IntoIter<u32>>,
    curr_selection: u32,

    //Atlas supports keys A-Z, Blank (# is the same tile), and Null (with the '-' key)
    game_tiles: Asset<Atlas>,
    token_tiles: Asset<Atlas>,
    selectable_animator: Animator,
    soft_click: Asset<Sound>,
    click: Asset<Sound>,

    winner: PlayerType,
}

//This is here to prevent silly warning about flags
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
        let click_soft = "SoftClick.wav";
        let click_hard = "Click.wav";

        //Help text
        let move_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Arrow Keys to move, 0/1/2-end game", &FontStyle::new(20.0, Color::BLACK), )}));
        let action_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Up/Down-Scroll Left/Right-Aim + Enter", &FontStyle::new(20.0, Color::BLACK), )}));
        let end_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Ending turn... 0/1/2-end game", &FontStyle::new(20.0, Color::BLACK), )}));

        //Ability Labels
        //Wraith
        let drain_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Drain", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let decoy_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Decoy", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let rend_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Rend", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let drain_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Drain", &FontStyle::new(16.0, Color::WHITE), )}));
        let decoy_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Decoy", &FontStyle::new(16.0, Color::WHITE), )}));
        let rend_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Rend", &FontStyle::new(16.0, Color::WHITE), )}));
        //Support
        let bio_grey =  Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Bio", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let shield_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Shield", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let renew_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Renew", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let bio_white =  Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Bio", &FontStyle::new(16.0, Color::WHITE), )}));
        let shield_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Shield", &FontStyle::new(16.0, Color::WHITE), )}));
        let renew_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Renew", &FontStyle::new(16.0, Color::WHITE), )}));
        //Assault
        let pierce_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Pierce", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let grenade_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Grenade", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let airraid_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Air Raid", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let pierce_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Pierce", &FontStyle::new(16.0, Color::WHITE), )}));
        let grenade_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Grenade", &FontStyle::new(16.0, Color::WHITE), )}));
        let airraid_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Air Raid", &FontStyle::new(16.0, Color::WHITE), )}));
        //Trapper
        let caltrop_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Caltrop", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let spear_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Spear", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let cage_grey = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Cage", &FontStyle::new(16.0, Color::from_rgba(132, 126, 135, 255.0)), )}));
        let caltrop_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Caltrop", &FontStyle::new(16.0, Color::WHITE), )}));
        let spear_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Spear", &FontStyle::new(16.0, Color::WHITE), )}));
        let cage_white = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Cage", &FontStyle::new(16.0, Color::WHITE), )}));

        //Action help text
        //Support
        let bio_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Radial healing pulse, bad for monsters", &FontStyle::new(20.0, Color::BLACK), )}));
        let shield_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Stationary shield bubble on allies", &FontStyle::new(20.0, Color::BLACK), )}));
        let renew_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Radial revive, heal, and restore", &FontStyle::new(20.0, Color::BLACK), )}));
        //Assault
        let pierce_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Fire a shot that penetrates barriers", &FontStyle::new(20.0, Color::BLACK), )}));
        let grenade_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Throw an incendiary grenade", &FontStyle::new(20.0, Color::BLACK), )}));
        let airraid_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Air strike that hits randomly in area", &FontStyle::new(20.0, Color::BLACK), )}));
        //Trapper
        let caltrop_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Throw caltrops that damage on-contact", &FontStyle::new(20.0, Color::BLACK), )}));
        let spear_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Fire a long range grappling spear", &FontStyle::new(20.0, Color::BLACK), )}));
        let cage_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Deploy a trapping forcefield on you", &FontStyle::new(20.0, Color::BLACK), )}));
        //Wraith
        let drain_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Siphon life from a target", &FontStyle::new(20.0, Color::BLACK), )}));
        let decoy_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Freeze adjacent targets and flee", &FontStyle::new(20.0, Color::BLACK), )}));
        let rend_help = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Cripple surrounding targets", &FontStyle::new(20.0, Color::BLACK), )}));

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
        //Stat Labels
        let sat_label = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("SAT", &FontStyle::new(14.0, Color::WHITE), )}));
        let elder_label = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("GOD", &FontStyle::new(14.0, Color::WHITE),)}));
        //Menu Words
        let hp_label = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("HP:", &FontStyle::new(14.0, Color::WHITE), )}));
        let move_label = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Move:", &FontStyle::new(14.0, Color::WHITE), )}));
        let team_label = Asset::new(Font::load(font_mononoki).and_then(|font| {
            font.render("Team:", &FontStyle::new(14.0, Color::WHITE),)}));

        //Create players
        let wraith = Entity::new_char(ClassType::Wraith, PlayerType::Player1, 1,
                                      Vector::new(9,11), false)
                                        .expect("Cannot create Wraith game::new.");
        let support = Entity::new_char(ClassType::Support, PlayerType::Player2, 1,
                                       Vector::new(6,4), false)
            .expect("Cannot create Support game::new.");
        let assault = Entity::new_char(ClassType::Assault, PlayerType::Player2, 1,
                                       Vector::new(9,3), false)
            .expect("Cannot create Assault game::new.");
        let trapper = Entity::new_char(ClassType::Trapper, PlayerType::Player2, 1,
                                       Vector::new(12,4), false)
            .expect("Cannot create Trapper game::new.");

        //Player Vector
        let player_ref = vec![wraith, support, assault, trapper];
        //Player Turn order, there must be no elements greater than `player_ref.len()-1`
        let mut turn_order = vec![1,0,2,0,3].into_iter().cycle();
        let curr_player = turn_order.next().expect("Cannot find first player");
        //Find first player's stats
        let moves = *player_ref[curr_player].get_stats()?.get_speed() as u32;
        let actions = *player_ref[curr_player].get_stats()?.get_actions() as u32;

        //Setup ability selection
        let mut selections = vec![0,1,2].into_iter().cycle();
        let curr_selection = selections.next().expect("Cannot find first selection");

        //Setup Sound Asssets
        let soft_click = Asset::new(Sound::load(click_soft));
        let click = Asset::new(Sound::load(click_hard));

        //Setup psuedo animation
        let animation_keys = vec!["S1".to_string(),"S2".to_string(),"S3".to_string(),"S4".to_string(),"S5".to_string()].into_iter().cycle();

        //Setup direction tracker
        let mut directions = vec![Direction::Up, Direction::Right, Direction::Down, Direction::Left].into_iter().cycle();

        Ok(Self {
            game_background: Asset::new(Image::load(background)),
            game_overlay: Asset::new(Image::load(overlay)),
            move_help, action_help, end_help,
            controls_text, class_text, info_text, abilities_text,
            underline: Asset::new(Image::load(underline)),
            //[Press] Labels
            move_text, action_text, end_text,
            move_grey, action_grey, end_grey,
            //Menu Labels
            hp_label, move_label, team_label, sat_label, elder_label,

            //Class Ability Labels
            //Wraith
            drain_grey, decoy_grey, rend_grey,
            drain_white, decoy_white, rend_white,
            drain_help, decoy_help, rend_help,
            //Support
            bio_grey, shield_grey, renew_grey,
            bio_white, shield_white, renew_white,
            bio_help, shield_help, renew_help,
            //Assault
            pierce_grey, grenade_grey, airraid_grey,
            pierce_white, grenade_white, airraid_white,
            pierce_help, grenade_help, airraid_help,
            //Trapper
            caltrop_grey, spear_grey, cage_grey,
            caltrop_white, spear_white, cage_white,
            caltrop_help, spear_help, cage_help,

            game_board: GameBoard::new().expect("Failed to load GameBoard in scenes::game::ElderGame::new"),
            player_ref,
            turn_order,
            curr_player,

            //Turn control data
            end_flag: false,
            action_state: ActionType::Move,
            curr_dir: directions.next().expect("Cannot load initial direction in scenes::game::ElderGame::new"),
            directions: directions,
            moves, actions,
            selections, curr_selection,

            game_tiles: Asset::new(Atlas::load(atlas_index)),
            token_tiles: Asset::new(Atlas::load(game_atlas_index)),
            selectable_animator: Animator::new(animation_keys, 0.17)?,

            soft_click, click,

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
        if kb[Key::M] == Pressed && self.moves > 0   {
            self.click.execute(|music| { music.play() })?;
            self.action_state = ActionType::Move;
        }
        else if kb[Key::M] == Pressed { self.soft_click.execute(|music| { music.play() })?; }

        if kb[Key::A] == Pressed && self.actions > 0 {
            self.click.execute(|music| { music.play() })?;
            self.curr_selection = 0; //Selection should always be the first option to start
            self.selections = vec![1,2,0].into_iter().cycle(); //Must also reset as if 0 was chosen
            self.curr_dir = Direction::Up; //Dir should always start at up
            self.directions = vec![Direction::Right, Direction::Down, Direction::Left, Direction::Up].into_iter().cycle(); //Must also reset as if up was chosen
            self.action_state = ActionType::Action;
        } else if kb[Key::A] == Pressed { self.soft_click.execute(|music| { music.play() })?; }

        if kb[Key::E] == Pressed                     { self.action_state = ActionType::End;}

        //Only accept commands when the player can do something
        match self.action_state {
            ActionType::Move => { // Default to this state so players are not forced to explicitly end and no cycles are created
                if self.moves > 0 {
                    if kb[Key::Up] == Pressed { moved = self.try_move(Vector::new(curr_loc.x, curr_loc.y - 1.0))?; }
                    else if kb[Key::Left] == Pressed { moved = self.try_move(Vector::new(curr_loc.x - 1.0, curr_loc.y))?;}
                    else if kb[Key::Down] == Pressed { moved = self.try_move(Vector::new(curr_loc.x, curr_loc.y + 1.0))?;}
                    else if kb[Key::Right] == Pressed { moved = self.try_move(Vector::new(curr_loc.x + 1.0, curr_loc.y))?;}
                } else {
                    if kb[Key::Up] == Pressed { self.soft_click.execute(|music| { music.play() })?; }
                    else if kb[Key::Left] == Pressed { self.soft_click.execute(|music| { music.play() })?; }
                    else if kb[Key::Down] == Pressed { self.soft_click.execute(|music| { music.play() })?; }
                    else if kb[Key::Right] == Pressed { self.soft_click.execute(|music| { music.play() })?; }
                }
            },
            ActionType::Action => {
                if self.actions > 0 {
                    if kb[Key::Up] == Pressed { self.prev_selection()?; } //Action selection
                    else if kb[Key::Down] == Pressed { self.next_selection()?;}

                    if kb[Key::Left] == Pressed { self.prev_direction()?; } //Direction changing
                    else if kb[Key::Right] == Pressed { self.next_direction()?;}

                    if kb[Key::Return] == Pressed {
                        //Check to see if a player is allowed to use the selected ability and use it if so
                        if self.player_ref[self.curr_player].can_act(self.curr_selection + 1, &self.game_board, &self.player_ref)? {
                            self.click.execute(|music| { music.play() })?;
                            self.player_ref[self.curr_player].act(self.curr_selection + 1, self.curr_dir, &self.game_board, &self.player_ref)?;
                            self.actions -= 1;
                        } else { self.soft_click.execute(|music| { music.play() })?; }
                    }
                } else { //Being in the action state with no actions is nonsensical and forbidden
                    self.action_state = ActionType::Move;
                }
            },
            ActionType::End => {
                self.end_flag = true;
                self.next_turn()?;
            }
        }

        if moved {
            moved = false;
            self.moves -= 1;
        }
        if acted {
            acted = false;
            self.actions -= 1;
        }

        if kb[Key::Key0] == Pressed {
            self.winner = PlayerType::Undetermined;
            retval = SceneReturn::Finished;
            self.reset()?;
        }
        if kb[Key::Key1] == Pressed {
            self.winner = PlayerType::Player1;
            retval = SceneReturn::Finished;
            self.reset()?;
        }
        if kb[Key::Key2] == Pressed {
            self.winner = PlayerType::Player2;
            retval = SceneReturn::Finished;
            self.reset()?;
        }

        Ok(retval)
    }

    /// Draw stuff on the screen
    pub fn draw(&mut self, window: &mut Window) -> Result<()> {
        let window_center = Vector::new(window.screen_size().x as i32 / 2, window.screen_size().y as i32 / 2);
        //This must be called to ensure that 'anim_key' is always the correct key of the animation to draw
        let anim_key = self.selectable_animator.next_if_not(window.current_fps())?.as_str();

        // Draw the frame and overlay
        draw_ex_with_center(window, &mut self.game_background, window_center, Transform::IDENTITY, 1.0)?;
        draw_ex_with_center(window, &mut self.game_overlay, window_center, Transform::IDENTITY, 2.0)?;

        // Draw GameBoard, calculates coordinates from the center for a 19x15 board of 40x40 pixels
        for row in self.game_board.get_board()? {
            for cell in row {
                let tile_key = cell.get_land()?.key().expect("No known key for tile.");;
                let cond_key = cell.get_cond()?.key().expect("No known key for tile.");
                let pos = cell.get_pos().expect("Failed to get cell position game::draw");

                //Draw land
                draw_ex_atlas_with_center(window, &mut self.game_tiles,
                                       Vector::new(window_center.x - 380.0 + (40.0 * pos.x) + 20.0,
                                                   window_center.y - 300.0 + (40.0 * pos.y) + 20.0),
                                          Transform::IDENTITY, 3.0, tile_key)?;

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

        //Get Player Info and calculate current bar size
        let player_team = self.player_ref[self.curr_player].get_player()?;

        let player_max_hp = *self.player_ref[self.curr_player].get_stats()?.get_hp() as f32;
        let player_hp = *self.player_ref[self.curr_player].get_curr_stats()?.get_hp() as f32;
        let full_hp_px = 85.0;
        let curr_hp_px: f32 = (player_hp / player_max_hp) * full_hp_px;

        let player_max_moves = *self.player_ref[self.curr_player].get_stats()?.get_speed() as f32;
        let player_moves = self.moves as f32;
        let full_mv_px = 55.0;
        let curr_mv_px: f32 = (player_moves / player_max_moves) * full_mv_px;

        //Draw Info Menu Labels
        draw_ex_with_center(window, &mut self.hp_label, Vector::new(window_center.x + 260.0, window_center.y - 201.0),
                            Transform::IDENTITY, 8.041)?;
        draw_ex_with_center(window, &mut self.move_label, Vector::new(window_center.x + 274.0, window_center.y - 171.0),
                            Transform::IDENTITY, 8.042)?;
        draw_ex_with_center(window, &mut self.team_label, Vector::new(window_center.x + 274.0, window_center.y - 141.0),
                            Transform::IDENTITY, 8.043)?;
        //Draw Menu Items
        //Team Labels
        match player_team {
            PlayerType::Player1 => {
                draw_ex_with_center(window, &mut self.elder_label, Vector::new(window_center.x + 334.0, window_center.y - 141.0),
                                    Transform::IDENTITY, 8.044)?;},
            PlayerType::Player2 => {
                draw_ex_with_center(window, &mut self.sat_label, Vector::new(window_center.x + 334.0, window_center.y - 141.0),
                                    Transform::IDENTITY, 8.045)?;},
            _ => {/*Ignore Undetermined*/},
        };

        //Draw Move and HP meters
        let hp_bar = Rectangle::new(Vector::new(window_center.x + 282.0, window_center.y - 208.0), (full_hp_px, 13.0));
        let curr_hp_bar = Rectangle::new(Vector::new(window_center.x + 282.0, window_center.y - 208.0), (curr_hp_px, 13.0));
        window.draw_ex(&hp_bar, Col(Color::RED.with_alpha(0.5)), Transform::IDENTITY, 8.046);
        window.draw_ex(&curr_hp_bar, Col(Color::RED), Transform::IDENTITY, 8.047);
        let move_bar = Rectangle::new(Vector::new(window_center.x + 310.0, window_center.y - 178.0), (full_mv_px, 13.0));
        let curr_move_bar = Rectangle::new(Vector::new(window_center.x + 310.0, window_center.y - 178.0), (curr_mv_px, 13.0));
        window.draw_ex(&move_bar, Col(Color::BLUE.with_alpha(0.5)), Transform::IDENTITY, 8.047);
        window.draw_ex(&curr_move_bar, Col(Color::BLUE), Transform::IDENTITY, 8.048);

        // Draw State Indicator
        let y_offset = match self.action_state {
            ActionType::Move => { 186.0 },
            ActionType::Action => { 216.0 },
            ActionType::End => { 246.0 }
        };
        draw_ex_with_center(window, &mut self.underline, Vector::new(window_center.x - 301.0, window_center.y + y_offset),
                            Transform::IDENTITY, 8.05)?;

        //Render the white button if the player can do the action
        let mut move_text = &mut self.move_grey;
        let mut action_text = &mut self.action_grey;
        let mut end_text = &mut self.end_grey;
        if self.moves > 0   { move_text = &mut self.move_text; }
        if self.actions > 0 { action_text = &mut self.action_text; }
        if !self.end_flag   { end_text = &mut self.end_text; }
        // Draw label text items, should always render on top to show the state the game is in
        draw_ex_with_center(window, move_text, Vector::new(window_center.x - 303.0, window_center.y + 175.0), Transform::IDENTITY, 8.11)?;
        draw_ex_with_center(window, action_text, Vector::new(window_center.x - 303.0, window_center.y + 205.0), Transform::IDENTITY, 8.12)?;
        draw_ex_with_center(window, end_text, Vector::new(window_center.x - 303.0, window_center.y + 235.0), Transform::IDENTITY, 8.13)?;

        // Draw Actions
        let mut action_1;
        let mut action_2;
        let mut action_3;
        //Decide which actions can be taken
        match self.player_ref[self.curr_player].get_class()? {
            ClassType::Support  => {
                action_1 = &mut self.bio_grey;
                action_2 = &mut self.shield_grey;
                action_3 = &mut self.renew_grey;
                if self.actions > 0 {
                    if self.player_ref[self.curr_player].can_act(1, &self.game_board, &self.player_ref)? { action_1 = &mut self.bio_white;}
                    if self.player_ref[self.curr_player].can_act(2, &self.game_board, &self.player_ref)? { action_2 = &mut self.shield_white; }
                    if self.player_ref[self.curr_player].can_act(3, &self.game_board, &self.player_ref)? { action_3 = &mut self.renew_white; }
                }
            },
            ClassType::Assault  => {
                action_1 = &mut self.pierce_grey;
                action_2 = &mut self.grenade_grey;
                action_3 = &mut self.airraid_grey;
                if self.actions > 0 {
                    if self.player_ref[self.curr_player].can_act(1, &self.game_board, &self.player_ref)? { action_1 = &mut self.pierce_white;}
                    if self.player_ref[self.curr_player].can_act(2, &self.game_board, &self.player_ref)? { action_2 = &mut self.grenade_white; }
                    if self.player_ref[self.curr_player].can_act(3, &self.game_board, &self.player_ref)? { action_3 = &mut self.airraid_white; }
                }
            },
            ClassType::Trapper  => {
                action_1 = &mut self.caltrop_grey;
                action_2 = &mut self.spear_grey;
                action_3 = &mut self.cage_grey;
                if self.actions > 0 {
                    if self.player_ref[self.curr_player].can_act(1, &self.game_board, &self.player_ref)? { action_1 = &mut self.caltrop_white;}
                    if self.player_ref[self.curr_player].can_act(2, &self.game_board, &self.player_ref)? { action_2 = &mut self.spear_white; }
                    if self.player_ref[self.curr_player].can_act(3, &self.game_board, &self.player_ref)? { action_3 = &mut self.cage_white; }
                }
            },
            ClassType::Wraith   => {
                action_1 = &mut self.drain_grey;
                action_2 = &mut self.decoy_grey;
                action_3 = &mut self.rend_grey;
                if self.actions > 0 {
                    if self.player_ref[self.curr_player].can_act(1, &self.game_board, &self.player_ref)? { action_1 = &mut self.drain_white;}
                    if self.player_ref[self.curr_player].can_act(2, &self.game_board, &self.player_ref)? { action_2 = &mut self.decoy_white; }
                    if self.player_ref[self.curr_player].can_act(3, &self.game_board, &self.player_ref)? { action_3 = &mut self.rend_white; }
                }
            },
            _c                   => { panic!("Attempted to render unsupported class abilities, {:?}", _c) }
        };
        draw_ex_with_center(window, action_1, Vector::new(window_center.x + 303.0, window_center.y + 175.0), Transform::IDENTITY, 8.14)?;
        draw_ex_with_center(window, action_2, Vector::new(window_center.x + 303.0, window_center.y + 205.0), Transform::IDENTITY, 8.15)?;
        draw_ex_with_center(window, action_3, Vector::new(window_center.x + 303.0, window_center.y + 235.0), Transform::IDENTITY, 8.16)?;

        //Draw an underline under selected option
        if self.action_state == ActionType::Action {
            // Draw State Indicator
            let y_offset = match self.curr_selection {
                0 => { 186.0 },
                1 => { 216.0 },
                2 => { 246.0 },
                _ => { panic!("Invalid underline index found for Action selection.") }
            };
            draw_ex_with_center(window, &mut self.underline, Vector::new(window_center.x + 301.0, window_center.y + y_offset),
                                Transform::IDENTITY, 8.05)?;
        }

        //Draw appropriate general help text
        let help = match self.action_state{
            ActionType::Move => {&mut self.move_help},
            ActionType::Action => {&mut self.action_help},
            ActionType::End => {&mut self.end_help}
        };
        draw_ex_with_center(window, help, Vector::new(window_center.x, window_center.y + 286.0),
                                             Transform::IDENTITY, 8.4)?;

        //Draw action help text
        if self.action_state == ActionType::Action {
            //Decide which help text to render
            let action_help = match self.player_ref[self.curr_player].get_class()? {
                ClassType::Support  => {
                    match self.curr_selection {
                        0 => &mut self.bio_help,
                        1 => &mut self.shield_help,
                        2 => &mut self.renew_help,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                ClassType::Assault  => {
                    match self.curr_selection {
                        0 => &mut self.pierce_help,
                        1 => &mut self.grenade_help,
                        2 => &mut self.airraid_help,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                ClassType::Trapper  => {
                    match self.curr_selection {
                        0 => &mut self.caltrop_help,
                        1 => &mut self.spear_help,
                        2 => &mut self.cage_help,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                ClassType::Wraith   => {
                    match self.curr_selection {
                        0 => &mut self.drain_help,
                        1 => &mut self.decoy_help,
                        2 => &mut self.rend_help,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                _c                   => { panic!("Attempted to render unsupported class abilities, {:?}", _c) }
            };
            draw_ex_with_center(window, action_help, Vector::new(window_center.x, window_center.y - 286.0),
                                Transform::IDENTITY, 8.41)?;
        }

        //Draw selectable animation
        // This targeting logic must match targeting logic used in entities targeting logic to be correct
        if self.action_state == ActionType::Action {
            //Decide which tiles to put the animation on
            let selectable_coordinates = match self.player_ref[self.curr_player].get_class()? {
                ClassType::Support  => {
                    match self.curr_selection {
                        0 => self.player_ref[self.curr_player].adjacent_range(3, &self.game_board, &self.player_ref)?,
                        1 => self.player_ref[self.curr_player].list_range_ally(&self.game_board, &self.player_ref)?,
                        2 => self.player_ref[self.curr_player].adjacent_range(2, &self.game_board, &self.player_ref)?,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                ClassType::Assault  => {
                    match self.curr_selection {
                        0 => self.player_ref[self.curr_player].directed_line_range(3, self.curr_dir, &self.game_board, &self.player_ref)?,
                        1 => self.player_ref[self.curr_player].directed_line_radial_cast(3, 1, self.curr_dir, &self.game_board, &self.player_ref)?,
                        2 => self.player_ref[self.curr_player].directed_line_radial(3, 3, self.curr_dir, &self.game_board, &self.player_ref)?,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                ClassType::Trapper  => {
                    match self.curr_selection {
                        0 => self.player_ref[self.curr_player].directed_line_radial(1, 1, self.curr_dir, &self.game_board, &self.player_ref)?,
                        1 => self.player_ref[self.curr_player].directed_line_cast(6, self.curr_dir, &self.game_board, &self.player_ref)?,
                        2 => self.player_ref[self.curr_player].adjacent_shell(3,  &self.game_board, &self.player_ref)?,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                ClassType::Wraith   => {
                    match self.curr_selection {
                        0 => self.player_ref[self.curr_player].adjacent_range(1, &self.game_board, &self.player_ref)?,
                        1 => self.player_ref[self.curr_player].adjacent_range(1, &self.game_board, &self.player_ref)?,
                        2 => self.player_ref[self.curr_player].adjacent_range(1, &self.game_board, &self.player_ref)?,
                        _ => panic!("Tried to draw invalid ability.")
                    }
                },
                _c                   => { panic!("Attempted to render unsupported action highlight, {:?}", _c) }
            };
            //Draw on tiles that are affected by an ability
            for coordinate in selectable_coordinates {
                draw_ex_atlas_with_center(window, &mut self.token_tiles,
                                          Vector::new(window_center.x - 380.0 + (40.0 * coordinate.x) + 23.0,
                                                      window_center.y - 300.0 + (40.0 * coordinate.y) + 18.0), Transform::IDENTITY, 8.5, anim_key)?;
            }

        }

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
        self.moves = *self.player_ref[self.curr_player].get_stats()?.get_speed() as u32;
        self.actions = *self.player_ref[self.curr_player].get_stats()?.get_actions() as u32;
        self.action_state = ActionType::Move;
        self.end_flag = false;
        self.game_board.decrement_temp_cond_counters()?;

        Ok(())
    }

    /// Selects the next ability index between 0-2 to represent the first, second, and third options
    pub fn next_selection(&mut self) -> Result<()> {
        self.curr_selection = self.selections.next().expect("Cannot find next ability selection index.");
        Ok(())
    }

    /// Selects the previous ability index between 0-2 to represent the first, second, and third options
    pub fn prev_selection(&mut self) -> Result<()> {
        //Iterating twice through a 3 element cycle is equal to going backwards once
        self.next_selection()?;
        self.next_selection()?;
        Ok(())
    }

    /// Selects the next direction and sets the current direction
    pub fn next_direction(&mut self) -> Result<()> {
        self.curr_dir = self.directions.next().expect("Cannot find next direction.");
        Ok(())
    }

    /// Selects the previous direction and sets the current direction
    pub fn prev_direction(&mut self) -> Result<()> {
        //Iterating thrice through a 4 element cycle is equal to going backwards once
        self.next_direction()?;
        self.next_direction()?;
        self.next_direction()?;
        Ok(())
    }

    ///Resets the game
    pub fn reset(&mut self) -> Result<()> {
        //Create players
        let wraith = Entity::new_char(ClassType::Wraith, PlayerType::Player1, 1,
                                      Vector::new(9,11), false)
            .expect("Cannot create Wraith game::new.");
        let support = Entity::new_char(ClassType::Support, PlayerType::Player2, 1,
                                       Vector::new(6,4), false)
            .expect("Cannot create Support game::new.");
        let assault = Entity::new_char(ClassType::Assault, PlayerType::Player2, 1,
                                       Vector::new(9,3), false)
            .expect("Cannot create Assault game::new.");
        let trapper = Entity::new_char(ClassType::Trapper, PlayerType::Player2, 1,
                                       Vector::new(12,4), false)
            .expect("Cannot create Trapper game::new.");

        //Player Vector
        let player_ref = vec![wraith, support, assault, trapper];
        //There must be no number greater than `player_ref.len()-1`
        let mut turn_order = vec![1,0,2,0,3].into_iter().cycle();
        let curr_player = turn_order.next().expect("Cannot find first player");
        //Find first player's stats
        let moves = *player_ref[curr_player].get_stats()?.get_speed() as u32;
        let actions = *player_ref[curr_player].get_stats()?.get_actions() as u32;

        //Setup ability selection
        let mut selections = vec![0,1,2].into_iter().cycle();
        let curr_selection = selections.next().expect("Cannot find first selection");

        self.player_ref = player_ref;
        self.turn_order = turn_order;
        self.curr_player = curr_player;
        self.moves = moves;
        self.actions = actions;
        self.game_board = GameBoard::new()?;
        self.selections = selections;
        self.curr_selection = curr_selection;

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