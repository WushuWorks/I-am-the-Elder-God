/*
Here we write the character classes that inhabit the field
*/
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

/// Describes the attributes of a particular class
#[allow(unused)]
#[derive(Clone, Copy)]
struct Attributes {
    hp: i32,
    speed: u32,
    armor: i32,
    power: i32,
}

#[allow(unused)]
impl Attributes {
    /// Initialize universal stats
    pub fn new() -> Self { Self{hp: 1, speed: 0, armor: 1, power: 1} }
    /// Sets stats for a class
    pub fn set_class(&mut self, class: &ClassType) -> Result<Self> {
        let hp: i32;
        let speed: u32;
        let armor: i32;
        let power: i32;

        //Attribute allocation
        match class {
                    ClassType::Support => {
                        hp = 100;
                        speed = 3;
                        armor = 2;
                        power = 2;
                    },
                    ClassType::Assault => {
                        hp = 90;
                        speed = 4;
                        armor = 1;
                        power = 4;
                    },
                    ClassType::Trapper => {
                        hp = 100;
                        speed = 3;
                        armor = 2;
                        power = 2;
                    }
                    ClassType::Wraith => {
                        hp = 100;
                        speed = 4;
                        armor = 2;
                        power = 2;
                    },
                    ClassType::Kraken => {
                        hp = 250;
                        speed = 3;
                        armor = 4;
                        power = 4;
                    },
                    ClassType::Elder => {
                        hp = 500;
                        speed = 3;
                        armor = 2;
                        power = 2;
                    }
                    ClassType::NPC => {
                        hp = 1;
                        speed = 1;
                        armor = 1;
                        power = 1;
                    }
        }

        Ok(Self{hp, speed, armor, power,})
    }
    pub fn set_custom_stats(&mut self, hp: i32, speed: u32, armor: i32, power: i32) -> Result<Self> {Ok(Self{hp, speed, armor, power})}
    pub fn get_hp(&self) -> &i32 { &self.hp }
    pub fn get_speed(&self) -> &u32{ &self.speed }
    pub fn get_armor(&self) -> &i32{ &self.armor }
    pub fn get_power(&self) -> &i32{ &self.power }
}

/// This models the most universal class
#[allow(unused)]
#[derive(Clone, Copy)]
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
    pub fn new_npc(player: PlayerType, hp: i32, speed: u32, armor: i32, power: i32, pos: Vector, invincible: bool, tangible: bool) -> Result<Self> {
        Ok(Self{
            player,
            class: ClassType::NPC,
            stats: Attributes::new().set_custom_stats(hp, speed, armor, power).expect("Cannot create npc with given stats"),
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
    pub fn get_player(&self) -> Result<(&PlayerType)> { Ok((&self.player)) }

}










/// Class abilities
trait Wraith {}
trait Kraken {}
trait Elder {}
trait Support {}
trait Assault {}
trait Trapper {}

/*
/// Try to move to any given location
fn teleport(new_location: Vector) -> Result<()>;
/// Move according to input
fn walk(keyboard: &Keyboard, gamepad: &Gamepad) -> Result<()>;
*/