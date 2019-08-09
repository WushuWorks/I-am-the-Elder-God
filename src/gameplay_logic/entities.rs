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
    curr_hp: f32,
    speed: f32,
    armor: f32,
    power: f32,
    actions: f32,
    exp: f32,
}

#[allow(unused)]
impl Attributes {
    /// Initialize universal stats
    pub fn new() -> Self { Self{hp: 1.0, curr_hp: 1.0, speed: 0.0, armor: 1.0, power: 1.0, actions: 0.0, exp: 0.0} }
    /// Sets stats for a class
    pub fn set_class(&mut self, class: &ClassType) -> Result<Self> {
        let hp: f32;
        let curr_hp: f32;
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
                        hp = 250.0;
                        speed = 3.0;
                        armor = 4.0;
                        power = 4.0;
                        actions = 1.0;
                    },
                    ClassType::Elder => {
                        hp = 500.0;
                        speed = 3.0;
                        armor = 2.0;
                        power = 2.0;
                        actions = 2.0;
                    }
                    ClassType::NPC => {
                        hp = 1.0;
                        speed = 1.0;
                        armor = 1.0;
                        power = 1.0;
                        actions = 1.0;
                    }
        }
        curr_hp = hp;

        Ok(Self{hp, curr_hp, speed, armor, power, actions, exp: 0.0})
    }
    pub fn set_custom_stats(&mut self, hp: f32, curr_hp: f32, speed: f32, armor: f32, power: f32, actions: f32, exp: f32) -> Result<Self> {
        Ok(Self{hp, curr_hp, speed, armor, power, actions, exp})
    }
    pub fn get_hp(&self)      -> &f32 { &self.hp }
    pub fn get_curr_hp(&self) -> &f32 { &self.curr_hp }
    pub fn get_speed(&self)   -> &f32 { &self.speed }
    pub fn get_armor(&self)   -> &f32 { &self.armor }
    pub fn get_power(&self)   -> &f32 { &self.power }
    pub fn get_actions(&self) -> &f32 { &self.actions }
    pub fn get_exp(&self)     -> &f32 { &self.exp }
    //Set
    pub fn set_hp(&mut self, hp: f32)            { self.hp = hp }
    pub fn set_curr_hp(&mut self, curr_hp: f32)  { self.curr_hp = curr_hp }
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
    pos: Vector,
    invincible: bool,
    tangible: bool, //Will this Entity be pass-through?
}

#[allow(unused)]
impl Entity{
    /// Makes class-less character
    pub fn new_npc(player: PlayerType, hp: f32, curr_hp: f32, speed: f32, armor: f32, power: f32, actions: f32, exp: f32, pos: Vector, invincible: bool, tangible: bool) -> Result<Self> {
        Ok(Self{
            player,
            class: ClassType::NPC,
            stats: Attributes::new().set_custom_stats(hp, curr_hp, speed, armor, power, actions, exp).expect("Cannot create npc with given stats"),
            pos,
            invincible,
            tangible,
        })
    }
    /// Sets a new character
    pub fn new_char(class: ClassType, player: PlayerType, pos: Vector, invincible: bool) -> Result<Self>{
        Ok(Self{
            player,
            class,
            stats: Attributes::new().set_class(&class).expect("Cannot set class stats"),
            pos,
            invincible,
            tangible: true,
        })
    }
    /// Gets player info
    pub fn get_player(&self) -> Result<&PlayerType> { Ok(&self.player) }
    pub fn get_class(&self) -> Result<&ClassType> { Ok(&self.class) }
    pub fn get_tangible(&self) -> Result<bool> { Ok(self.tangible) }
    pub fn get_pos(&self) -> Result<Vector> { Ok(self.pos) }
    pub fn get_stats(&self) -> Result<&Attributes> { Ok(&self.stats) }
    /// Sets player info
    pub fn set_pos(&mut self, new_loc: Vector) -> Result<()> {
        self.pos = new_loc;
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
}



/// Class abilities
trait Wraith {}
trait Kraken {}
trait Elder {}
trait Support {}
trait Assault {}
trait Trapper {}