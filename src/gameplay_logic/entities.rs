/*
Here we write the character classes that inhabit the field
*/
use crate::gameplay_logic::game_board::GameBoard;
use crate::gameplay_logic::gameplay_type::*;

use quicksilver::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PlayerType {
    Player1,
    Player2,
    Undetermined,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(unused)]
pub enum ClassType {
    Support,
    Assault,
    Trapper,
    Wraith,
    Kraken,
    Elder,
    NPC,
}

impl ClassType {
    ///Map enum to an index string if possible
    pub fn key(&self) -> &str {
         match self {
             ClassType::Support => {"Support"},
             ClassType::Assault => {"Assault"},
             ClassType::Trapper => {"Trapper"},
             ClassType::Wraith => {"Wraith"},
             ClassType::Kraken => {"Kraken"},
             ClassType::Elder => {"Elder"},
             ClassType::NPC => {"-"} //This maps to a blank TerrainStatus
        }
    }
}

/// Describes the attributes of a particular class
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Attributes {
    hp: f32,
    speed: f32,
    armor: f32,
    power: f32,
    actions: f32,
    exp: f32,
}

#[allow(unused)]
impl Attributes {
    /// Initialize universal stats
    pub fn new() -> Self { Self{hp: 1.0, speed: 0.0, armor: 1.0, power: 1.0, actions: 0.0, exp: 0.0} }
    /// Sets stats for a class
    pub fn set_class(&mut self, class: &ClassType) -> Result<Self> {
        let hp: f32;
        let speed: f32;
        let armor: f32;
        let power: f32;
        let actions: f32;
        //Attribute allocation
        match class {
                    ClassType::Support => {
                        hp = 100.0;
                        speed = 3.0;
                        armor = 2.0;
                        power = 2.0;
                        actions = 1.0;
                    },
                    ClassType::Assault => {
                        hp = 90.0;
                        speed = 4.0;
                        armor = 1.0;
                        power = 4.0;
                        actions = 1.0;
                    },
                    ClassType::Trapper => {
                        hp = 100.0;
                        speed = 3.0;
                        armor = 2.0;
                        power = 2.0;
                        actions = 1.0;
                    }
                    ClassType::Wraith => {
                        hp = 100.0;
                        speed = 4.0;
                        armor = 2.0;
                        power = 2.0;
                        actions = 1.0;
                    },
                    ClassType::Kraken => {
                        hp = 200.0;
                        speed = 4.0;
                        armor = 3.0;
                        power = 4.0;
                        actions = 1.0;
                    },
                    ClassType::Elder => {
                        hp = 500.0;
                        speed = 4.0;
                        armor = 5.0;
                        power = 6.0;
                        actions = 2.0;
                    },
                    ClassType::NPC => {
                        hp = 1.0;
                        speed = 1.0;
                        armor = 1.0;
                        power = 1.0;
                        actions = 1.0;
                    }
        }

        Ok(Self{hp, speed, armor, power, actions, exp: 0.0})
    }
    /// Makes a new set of custom stats
    pub fn new_custom_stats(hp: f32, speed: f32, armor: f32, power: f32, actions: f32, exp: f32) -> Result<Self> {
        Ok(Self{hp, speed, armor, power, actions, exp})
    }
    pub fn get_hp(&self)      -> &f32 { &self.hp }
    pub fn get_speed(&self)   -> &f32 { &self.speed }
    pub fn get_armor(&self)   -> &f32 { &self.armor }
    pub fn get_power(&self)   -> &f32 { &self.power }
    pub fn get_actions(&self) -> &f32 { &self.actions }
    pub fn get_exp(&self)     -> &f32 { &self.exp }
    //Set
    pub fn set_hp(&mut self, hp: f32)            { self.hp = hp }
    pub fn set_speed(&mut self, speed: f32)      { self.speed = speed }
    pub fn set_armor(&mut self, armor: f32)      { self.armor = armor }
    pub fn set_power(&mut self, power: f32)      { self.power = power }
    pub fn set_actions(&mut self, actions: f32)  { self.actions = actions }
    pub fn set_exp(&mut self, exp: f32)          { self.exp = exp }
}

/// This models the most universal class
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    player: PlayerType,
    class: ClassType,
    stats: Attributes,
    curr_stats: Attributes,
    level: u32,
    pos: Vector,
    invincible: bool,
    tangible: bool, //Will this Entity be pass-through?
}

#[allow(unused)]
impl Entity{
    /// Makes a new character of ClassType
    pub fn new_char(class: ClassType, player: PlayerType, level: u32, pos: Vector, invincible: bool) -> Result<Self>{
        Ok(Self{
            player,
            class,
            stats: Attributes::new().set_class(&class).expect("Cannot set class stats"),
            curr_stats: Attributes::new().set_class(&class).expect("Cannot set class stats"),
            level, pos, invincible,
            tangible: true,
        })
    }
    /// Gets player info
    pub fn get_player(&self) -> Result<&PlayerType> { Ok(&self.player) }
    pub fn get_class(&self) -> Result<&ClassType> { Ok(&self.class) }
    pub fn get_tangible(&self) -> Result<bool> { Ok(self.tangible) }
    pub fn get_pos(&self) -> Result<Vector> { Ok(self.pos) }
    pub fn get_level(&self) -> Result<u32> { Ok(self.level) }
    pub fn get_stats(&self) -> Result<&Attributes> { Ok(&self.stats) }
    pub fn get_curr_stats(&self) -> Result<&Attributes> { Ok(&self.curr_stats) }
    /// Sets player info
    pub fn set_pos(&mut self, new_loc: Vector) -> Result<()> {
        self.pos = new_loc;
        Ok(())
    }
    pub fn set_stats(&mut self, hp: f32, speed: f32, armor: f32, power: f32, actions: f32, exp: f32) -> Result<()> {
        self.stats = Attributes::new_custom_stats(hp, speed, armor, power, actions, exp)?;
        Ok(())
    }

    /// Check to see if this entity can move into a given location
    pub fn can_move(&self, location: Vector, board: &GameBoard, players: &Vec<Entity>) -> Result<bool> {
        let mut movable = true; //assume truth and attempt to disprove
        let cell = board.get_board()?[location.y as usize][location.x as usize];
        let land = *cell.get_land()?;
        let cond = *cell.get_cond()?;

        if self.tangible { //If we are tangible we need to check for tangible barriers
            for player in players { //Check all players to see if there is a tangible player in location
                if player.get_pos()? == location {
                    movable = false;
                }
            }

            if land == Terrain::Empty { //Check for impassable Terrain types
                movable = false;
            }

            //Check for impassable TerrainStatus types
            match cond {
                TerrainStatus::Frozen     => movable = false,
                TerrainStatus::Shielded   => movable = false,
                TerrainStatus::Impassable => movable = false,
                _                         => {}
            }
        }

        // Offboard spaces should always be illegal moves
        if land == Terrain::Empty {
            movable = false;
        }

        Ok(movable)
    }

    /// Returns a string corresponding to the name of the passed ability number
    /// Accepts ability numbers 1-3 inclusively.
    pub fn act(&self, action_index: u32, _board: &GameBoard, _players: &Vec<Entity>) -> Result<&str> {
        let action = match self.class {
            ClassType::Support  => {
                match action_index {
                    1 => { Ok("Bio") },
                    2 => { Ok("Shield") },
                    3 => { Ok("Renew") },
                    _ => { panic!("Unknown Support Ability Number") }
                }
            },
            ClassType::Assault  => {
                match action_index {
                    1 => { Ok("Pierce") },
                    2 => { Ok("Grenade") },
                    3 => { Ok("Airstrike") },
                    _ => { panic!("Unknown Assault Ability Number") }
                }
            },
            ClassType::Trapper  => {
                match action_index {
                    1 => { Ok("Caltrop") },
                    2 => { Ok("Spear") },
                    3 => { Ok("Cage") },
                    _ => { panic!("Unknown Trapper Ability Number") }
                }
            },
            ClassType::Wraith   => {
                match action_index {
                    1 => { Ok("Drain") },
                    2 => { Ok("Decoy") },
                    3 => { Ok("Rend") },
                    _ => { panic!("Unknown Wraith Ability Number") }
                }
            },
            _                   => { panic!("Unsupported Class for abilities") }
        };

        action
    }

    /// Returns true if an ability from 1-3, inclusively, can be used. False otherwise
    pub fn can_act(&self, action_index: u32, _board: &GameBoard, _players: &Vec<Entity>) -> Result<bool> {
        let actable = match self.class {
            ClassType::Support  => {
                match action_index {
                    1 => { if self.level >= 1 { true } else { false } },
                    2 => { if self.level >= 2 { true } else { false } },
                    3 => { if self.level >= 3 { true } else { false } },
                    _ => { false }
                }
            },
            ClassType::Assault  => {
                match action_index {
                    1 => { if self.level >= 1 { true } else { false } },
                    2 => { if self.level >= 2 { true } else { false } },
                    3 => { if self.level >= 3 { true } else { false } },
                    _ => { false }
                }
            },
            ClassType::Trapper  => {
                match action_index {
                    1 => { if self.level >= 1 { true } else { false } },
                    2 => { if self.level >= 2 { true } else { false } },
                    3 => { if self.level >= 3 { true } else { false } },
                    _ => { false }
                }
            },
            ClassType::Wraith   => {
                match action_index {
                    1 => { true },
                    2 => { true },
                    3 => { true },
                    _ => { false }
                }
            },
            _                   => { false }
        };

        Ok(actable)
    }

    /// Returns true if the passed location is attackable, false otherwise
    pub fn can_attack(&self, location: Vector, board: &GameBoard, players: &Vec<Entity>) -> bool {
        let mut attackable = true; //assume truth and attempt to disprove
        let cell = board.get_board().unwrap()[location.y as usize][location.x as usize];
        let land = *cell.get_land().unwrap();
        let cond = *cell.get_cond().unwrap();

        // Offboard spaces should always be unselectable
        if land == Terrain::Empty {
            attackable = false;
        }

        attackable
    }

    /// Returns a list of coordinates adjacent to the player
    pub fn adjacent(&self, board: &GameBoard, players: &Vec<Entity>) -> Vec<Vector> {
        let mut adjacent = vec![];
        let up = Vector::new(self.pos.x - 1.0, self.pos.y);
        let down = Vector::new(self.pos.x + 1.0, self.pos.y);
        let left = Vector::new(self.pos.x, self.pos.y - 1.0);
        let right = Vector::new(self.pos.x, self.pos.y + 1.0);

        if self.can_attack( up, board, players) {
            adjacent.push(up);
        }
        if self.can_attack( down, board, players) {
            adjacent.push(down);
        }
        if self.can_attack( left, board, players) {
            adjacent.push(left);
        }
        if self.can_attack( right, board, players) {
            adjacent.push(right);
        }

        adjacent
    }
}