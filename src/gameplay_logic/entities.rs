/*
Here we write the character classes that inhabit the field
*/
use crate::gameplay_logic::game_board::GameBoard;
use crate::gameplay_logic::gameplay_type::*;
use crate::scenes::game::Direction;

use quicksilver::prelude::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ActionAbility {
    Bio, Shield, Renew,
    Pierce, Grenade, Airraid,
    Caltrop, Spear, Cage,
    Drain, Decoy, Rend,
}

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
                        hp = 200.0;
                        speed = 6.0;
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
    //Checked stat changes
    /// Add or subtract hp, checks for overflow, and prevents setting lower than 0.0
    /// Returns true on normal operation, false if flow errors occur or a negative hp
    /// was prevented
    fn add_hp(&mut self, hp: f32) -> bool    {
        let mut retval;

        //Check for over or underflows
        if self.hp + hp < std::f32::MIN || self.hp + hp > std::f32::MAX {
            println!("Under or overflow occured and ignored in damage calculation");
            retval = false;
        } else {
            if self.hp + hp <= 0.0 { //Prevent negative hp
                self.hp = 0.0;
                retval = false;
            } else {
                self.hp += hp;
            }
            retval = true;
        }
        retval
    }

    ///Takes a positive damage value and returns an armor reduced value
    pub fn armor_reduce(&mut self, dmg: f32) -> f32{
        let retval;

        //Reduces damage by armor% if not negative
        if dmg.is_sign_negative(){
            retval = 0.0;
        } else {
            retval = dmg - dmg * (*self.get_armor()/10.0);
        }

        retval
    }
}

///Status effects a player can have
#[derive(Debug, Clone, Copy)]
pub enum Status {
    Normal,
    Crippled,
}

/// This models the most universal class
#[derive(Debug, Clone, Copy)]
pub struct Entity {
    player: PlayerType,
    class: ClassType,
    stats: Attributes,
    curr_stats: Attributes,
    level: u32,
    pos: Vector,
    invincible: bool,
    status: Status,
    status_timer: u32,
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
            status: Status::Normal,
            status_timer: 0,
            tangible: true,
        })
    }
    /// Gets player info
    pub fn get_player(&self)            -> Result<&PlayerType> { Ok(&self.player) }
    pub fn get_class(&self)             -> Result<&ClassType> { Ok(&self.class) }
    pub fn get_tangible(&self)          -> Result<bool> { Ok(self.tangible) }
    pub fn get_pos(&self)               -> Result<Vector> { Ok(self.pos) }
    pub fn get_level(&self)             -> Result<u32> { Ok(self.level) }
    pub fn get_status(&self)            -> Result<Status> { Ok(self.status) }
    pub fn get_stats(&self)             -> Result<&Attributes> { Ok(&self.stats) }
    pub fn get_curr_stats(&mut self)    -> Result<&mut Attributes> { Ok(&mut self.curr_stats) }
    /// Sets player position
    pub fn set_pos(&mut self, new_loc: Vector) -> Result<()> {
        self.pos = new_loc;
        Ok(())
    }
    /// Sets players stats
    pub fn set_stats(&mut self, hp: f32, speed: f32, armor: f32, power: f32, actions: f32, exp: f32) -> Result<()> {
        self.stats = Attributes::new_custom_stats(hp, speed, armor, power, actions, exp)?;
        Ok(())
    }
    /// Sets a status with a duration
    pub fn set_status(&mut self, new_status: Status, duration: u32)  -> Result<()> {
        self.status = new_status;
        self.status_timer = duration;
        Ok(())
    }
    /// Decrements the status timer, and resets status to normal if it reaches 0
    pub fn decrement_timer(&mut self) {
        if self.status_timer > 0 {
            self.status_timer -= 1;
            if self.status_timer == 0 {
                self.set_status(Status::Normal, 0);
            }
        }
    }
    /// Adds or subtracts hp, preventing adding beyond max or lowering beyond 0.0
    /// Returns true on normal operation, false if overfill or flow error occurs
    pub fn add_checked_hp (&mut self, new_hp: f32) -> Result<bool> {
        //We must prevent overfilling hp
        let max_hp = *self.get_stats()?.get_hp();
        let hp = *self.get_curr_stats()?.get_hp();
        let diff = max_hp - hp;
        let retval;

        if new_hp > diff { //Max hp would be exceed
            self.get_curr_stats()?.add_hp(diff);
            retval = false;
        } else {
            retval = self.get_curr_stats()?.add_hp(new_hp);
        }

        Ok(retval)
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
            ClassType::Wraith   => { //Monsters always have access to all class abilities.
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

    /// Returns a string corresponding to the name of the passed ability number and a Vec of Vectors that it targets
    /// Accepts ability numbers 1-3 inclusively.
    pub fn act(&self, action_index: u32, direction: Direction, board: &GameBoard, players: &Vec<Entity>) -> Result<(Vec<Vector>, ActionAbility)> {
        let targets; //contains a list of targets for an ability

        let action = match self.class {
            ClassType::Support  => {
                match action_index {
                    1 => {
                        targets = self.adjacent_radial(3, board, players)?;;
                        ActionAbility::Bio
                    },
                    2 => {
                        targets = self.list_range_ally(board, players)?;
                        ActionAbility::Shield
                    },
                    3 => {
                        targets = self.adjacent_radial(2, board, players)?;
                        ActionAbility::Renew
                    },
                    _ => { panic!("Unknown Support Ability Number") }
                }
            },
            ClassType::Assault  => {
                match action_index {
                    1 => {
                        targets = self.directed_line_range(2, direction, board, players)?;
                        ActionAbility::Pierce
                    },
                    2 => {
                        targets = self.directed_line_radial_cast(3, 1, direction, board, players)?;
                        ActionAbility::Grenade
                    },
                    3 => {
                        targets = self.directed_line_radial(3, 3, direction, board, players)?;
                        ActionAbility::Airraid
                    },
                    _ => { panic!("Unknown Assault Ability Number") }
                }
            },
            ClassType::Trapper  => {
                match action_index {
                    1 => {
                        targets = self.directed_line_radial(1, 1, direction, board, players)?;
                        ActionAbility::Caltrop
                    },
                    2 => {
                        targets = self.directed_line_cast(6, direction, board, players)?;
                        ActionAbility::Spear
                    },
                    3 => {
                        targets = self.adjacent_shell(3, board, players)?;
                        ActionAbility::Cage
                    },
                    _ => { panic!("Unknown Trapper Ability Number") }
                }
            },
            ClassType::Wraith   => {
                match action_index {
                    1 => {
                        targets = self.adjacent_range(1, board, players)?;
                        ActionAbility::Drain
                    },
                    2 => {
                        targets = self.adjacent_range(1, board, players)?;
                        ActionAbility::Decoy
                    },
                    3 => {
                        targets = self.directed_line_radial(1,1, direction, board, players)?;
                        ActionAbility::Rend
                    },
                    _ => { panic!("Unknown Wraith Ability Number") }
                }
            },
            _                   => { panic!("Unsupported Class for abilities") }
        };

        Ok((targets, action))
    }
}

/// This impl contains targeting logic
impl Entity {
    /// Returns true if the passed location is attackable, false otherwise
    pub fn can_attack(&self, location: Vector, board: &GameBoard, _players: &Vec<Entity>) -> bool {
        let mut attackable = true; //assume truth and attempt to disprove
        let cell = board.get_board().unwrap()[location.y as usize][location.x as usize];
        let land = *cell.get_land().unwrap();
        let _cond = *cell.get_cond().unwrap();

        // Offboard spaces should always be unselectable
        if land == Terrain::Empty {
            attackable = false;
        }

        attackable
    }

    /// Returns a list of attackable coordinates of all characters controlled by a player
    pub fn list_range_ally(&self, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        //let player_pos = self.pos;

        for player in players { // For all players
            for row in board.get_board()? {
                for cell in row { //The cell is on the board
                    if self.can_attack(cell.get_pos()?, board, players) { //The cell can be attacked
                        if player.get_player()? == self.get_player()? && cell.get_pos()? == player.get_pos()? { //The player is an ally and is on the cell being examined
                            targetable.push(cell.get_pos()?);
                        }
                    }
                }
            }
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates adjacent to the player up to the range specified
    pub fn adjacent_range(&self, range: u32, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = self.pos;

        for row in board.get_board()? {
            for cell in row {
                let distance = cell.get_pos()? - player_pos;
                if distance.x.abs() + distance.y.abs() <= range as f32 { //The cell is in range of the player
                    if player_pos != cell.get_pos()? { //The cell is not on the player
                        if self.can_attack(cell.get_pos()?, board, players) { //The cell can be attacked
                            targetable.push(cell.get_pos()?);
                        }
                    }
                }
            }
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates adjacent to the player up to the range specified, including the player
    pub fn adjacent_radial(&self, range: u32, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = self.pos;

        for row in board.get_board()? {
            for cell in row {
                let distance = cell.get_pos()? - player_pos;
                if distance.x.abs() + distance.y.abs() <= range as f32 { //The cell is in range of the player
                    if self.can_attack(cell.get_pos()?, board, players) { //The cell can be attacked
                        targetable.push(cell.get_pos()?);
                    }
                }
            }
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates at the specified range around the player
    pub fn adjacent_shell(&self, range: u32, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = self.pos;

        for row in board.get_board()? {
            for cell in row {
                let distance = cell.get_pos()? - player_pos;
                if distance.x.abs() + distance.y.abs() == range as f32 { //The cell is in range of the player
                    if self.can_attack(cell.get_pos()?, board, players) { //The cell can be attacked
                        targetable.push(cell.get_pos()?);
                    }
                }
            }
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates from the passed coordinate outwards
    pub fn radial_range(&self, location: Vector, radius: u32, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = location;

        for row in board.get_board()? {
            for cell in row {
                let distance = cell.get_pos()? - player_pos;
                if distance.x.abs() + distance.y.abs() <= radius as f32 { //The cell is in range of the player
                    if self.can_attack(cell.get_pos()?, board, players) { //The cell can be attacked
                        targetable.push(cell.get_pos()?);
                    }
                }
            }
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates in the direction passed up to the range given
    pub fn directed_line_range(&self, range: u32, direction: Direction, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = self.pos;

        //Cannot look off board
        //We add 1 to `range` to make the call draw the correct range since we skip one element
        match direction {
            Direction::Up => {
                if player_pos.y != 0.0 { //Top edge
                    for elem in 1..range+1 {
                        if player_pos.y - elem as f32 <= 0.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x, player_pos.y - elem as f32), board, players) {
                            targetable.push(Vector::new(player_pos.x, player_pos.y - elem as f32));
                        }
                    }
                }
            },
            Direction::Down => {
                if player_pos.y != board.get_board()?.len() as f32 - 1.0 { //Bottom edge
                    for elem in 1..range+1 {
                        if player_pos.y + elem as f32 >= board.get_board()?.len() as f32 - 1.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x, player_pos.y + elem as f32), board, players) {
                            targetable.push(Vector::new(player_pos.x, player_pos.y + elem as f32));
                        }
                    }
                }
            },
            Direction::Left => {
                if player_pos.x != 0.0 { //Left Edge
                    for elem in 1..range+1 {
                        if player_pos.x - elem as f32 <= 0.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x - elem as f32, player_pos.y), board, players) {
                            targetable.push(Vector::new(player_pos.x - elem as f32, player_pos.y));
                        }
                    }
                }
            },
            Direction::Right => {
                if player_pos.x != board.get_board()?.first().unwrap().len() as f32 - 1.0 { //Right edge
                    for elem in 1..range+1 {
                        if player_pos.x + elem as f32 >= board.get_board()?.first().unwrap().len() as f32 - 1.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x + elem as f32, player_pos.y), board, players) {
                            targetable.push(Vector::new(player_pos.x + elem as f32, player_pos.y));
                        }
                    }
                }
            },
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates in the direction passed up to the range given, stopping at an un-move-into-able obstacle
    pub fn directed_line_cast(&self, range: u32, direction: Direction, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = self.pos;

        //Cannot look off board
        //We add 1 to `range` to make the call draw the correct range since we skip one element
        match direction {
            Direction::Up => {
                if player_pos.y != 0.0 { //Top edge
                    for elem in 1..range+1 {
                        if player_pos.y - elem as f32 <= 0.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x, player_pos.y - elem as f32), board, players) {
                            targetable.push(Vector::new(player_pos.x, player_pos.y - elem as f32));
                        } else {
                            if self.can_move(Vector::new(player_pos.x, player_pos.y - elem as f32), board, players)? {
                                targetable.push(Vector::new(player_pos.x, player_pos.y - elem as f32));
                            }
                            break;
                        }
                    }
                }
            },
            Direction::Down => {
                if player_pos.y != board.get_board()?.len() as f32 - 1.0 { //Bottom edge
                    for elem in 1..range+1 {
                        if player_pos.y + elem as f32 >= board.get_board()?.len() as f32 - 1.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x, player_pos.y + elem as f32), board, players) {
                            targetable.push(Vector::new(player_pos.x, player_pos.y + elem as f32));
                        } else {
                            if self.can_move(Vector::new(player_pos.x, player_pos.y + elem as f32), board, players)? {
                                targetable.push(Vector::new(player_pos.x, player_pos.y + elem as f32));
                            }
                            break;
                        }
                    }
                }
            },
            Direction::Left => {
                if player_pos.x != 0.0 { //Left Edge
                    for elem in 1..range+1 {
                        if player_pos.x - elem as f32 <= 0.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x - elem as f32, player_pos.y), board, players) {
                            targetable.push(Vector::new(player_pos.x - elem as f32, player_pos.y));
                        } else {
                            if self.can_move(Vector::new(player_pos.x - elem as f32, player_pos.y), board, players)? {
                                targetable.push(Vector::new(player_pos.x - elem as f32, player_pos.y));
                            }
                            break;
                        }
                    }
                }
            },
            Direction::Right => {
                if player_pos.x != board.get_board()?.first().unwrap().len() as f32 - 1.0 { //Right edge
                    for elem in 1..range+1 {
                        if player_pos.x + elem as f32 >= board.get_board()?.first().unwrap().len() as f32 - 1.0 { break } //Stop if we hit an edge
                        if self.can_attack(Vector::new(player_pos.x + elem as f32, player_pos.y), board, players) {
                            targetable.push(Vector::new(player_pos.x + elem as f32, player_pos.y));
                        } else {
                            if self.can_move(Vector::new(player_pos.x + elem as f32, player_pos.y), board, players)? {
                                targetable.push(Vector::new(player_pos.x + elem as f32, player_pos.y));
                            }
                            break;
                        }
                    }
                }
            },
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates up to the direction passed at the radius given, stopping at an un-move-into-able obstacle
    pub fn directed_line_radial_cast(&self, range: u32, radius: u32, direction: Direction, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable = vec![];
        let player_pos = self.pos;

        //Cannot look off board
        //We add 1 to `range` to make the call draw the correct range since we skip one element
        match direction {
            Direction::Up => {
                if player_pos.y != 0.0 { //Top edge
                    for elem in 1..range+1 {
                        //Obstacle encountered, go back one element
                        if !self.can_move(Vector::new(player_pos.x, player_pos.y - elem as f32), board, players)? {
                            targetable = self.radial_range(Vector::new(player_pos.x, player_pos.y - elem as f32 + 1.0), radius, board, players)?;
                            break;
                        }
                        //Final iteration, no obstacles
                        if elem == range {
                            targetable = self.radial_range(Vector::new(player_pos.x, player_pos.y - elem as f32), radius, board, players)?;
                            break;
                        }
                    }
                }
            },
            Direction::Down => {
                if player_pos.y != board.get_board()?.len() as f32 - 1.0 { //Bottom edge
                    for elem in 1..range+1 {
                        //Obstacle encountered, go back one element
                        if !self.can_move(Vector::new(player_pos.x, player_pos.y + elem as f32), board, players)? {
                            targetable = self.radial_range(Vector::new(player_pos.x, player_pos.y + elem as f32 - 1.0), radius, board, players)?;
                            break;
                        }
                        //Final iteration, no obstacles
                        if elem == range {
                            targetable = self.radial_range(Vector::new(player_pos.x, player_pos.y + elem as f32), radius, board, players)?;
                            break;
                        }

                    }
                }
            },
            Direction::Left => {
                if player_pos.x != 0.0 { //Left Edge
                    for elem in 1..range+1 {
                        //Obstacle encountered, go back one element
                        if !self.can_move(Vector::new(player_pos.x - elem as f32, player_pos.y), board, players)? {
                            targetable = self.radial_range(Vector::new(player_pos.x - elem as f32 + 1.0, player_pos.y), radius, board, players)?;
                            break;
                        }
                        //Final iteration, no obstacles
                        if elem == range {
                            targetable = self.radial_range(Vector::new(player_pos.x - elem as f32, player_pos.y), radius, board, players)?;
                            break;
                        }
                    }
                }
            },
            Direction::Right => {
                if player_pos.x != board.get_board()?.first().unwrap().len() as f32 - 1.0 { //Right edge
                    for elem in 1..range+1 {
                        //Obstacle encountered, go back one element
                        if !self.can_move(Vector::new(player_pos.x + elem as f32, player_pos.y), board, players)? {
                            targetable = self.radial_range(Vector::new(player_pos.x + elem as f32 - 1.0, player_pos.y), radius, board, players)?;
                            break;
                        }
                        //Final iteration, no obstacles
                        if elem == range {
                            targetable = self.radial_range(Vector::new(player_pos.x + elem as f32, player_pos.y), radius, board, players)?;
                            break;
                        }
                    }
                }
            },
        }

        Ok(targetable)
    }

    /// Returns a list of attackable coordinates in the direction passed at the range given at the radius given. Using player will not be included
    pub fn directed_line_radial(&self, range: u32, radius: u32, direction: Direction, board: &GameBoard, players: &Vec<Entity>) -> Result<Vec<Vector>> {
        let mut targetable ;
        let player_pos = self.pos;

        //Cannot look off board
        //We add 1 to `range` to make the call draw the correct range since we skip one element
        match direction {
            Direction::Up => {
                targetable = self.radial_range(Vector::new(player_pos.x, player_pos.y - range as f32), radius, board, players)?;
            },
            Direction::Down => {
                targetable = self.radial_range(Vector::new(player_pos.x, player_pos.y + range as f32), radius, board, players)?;
            },
            Direction::Left => {
                targetable = self.radial_range(Vector::new(player_pos.x - range as f32, player_pos.y), radius, board, players)?;
            },
            Direction::Right => {
                targetable = self.radial_range(Vector::new(player_pos.x + range as f32, player_pos.y), radius, board, players)?;
            },
        }

        //Caster is not included
        if targetable.contains(&player_pos) {
            let index =  targetable.iter().position(|x| *x == player_pos)
                .expect("Cannot find index of existing element entities::directed_line_radial");
            targetable.remove(index);
        }

        Ok(targetable)
    }

}