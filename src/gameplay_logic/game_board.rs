/*
This is the environment layer, which models the board and its contents
*/

use quicksilver::prelude::*;
use crate::gameplay_logic::gameplay_type::{Terrain, TerrainStatus, to_condition, to_terrain};
use crate::gameplay_logic::game_levels::Levels;

/// Generation logic adapted from [Quicksilver Rougelike](https://github.com/tomassedovic/quicksilver-roguelike)
/// Generates a map with the initial game state
/// Creates map by defining a Normal Plain and then modifying it to the end state
fn generate_map(size: Vector, level: Vec<String>, level_condition: Vec<String>) -> Vec<Vec<Cell>> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut loaded_level = level.iter();
    let mut loaded_level_conditions = level_condition.iter();
    let mut map = vec![vec![Cell::new();width];height];
    for y in 0..height {
        let mut row = vec![];
        for x in 0..width {

            let tile_key = loaded_level.next().expect("Tried to allocate wrong sized level");
            let tile_cond_key = loaded_level_conditions.next().expect("Tried to allocate wrong sized level");

            let land = to_terrain(tile_key).expect("Failed to translate key to Terrain game_board::generate_map");
            let condition = to_condition(tile_cond_key).expect("Failed to translate key to TerrainStatus game_board::generate_map");
            //Set strength of initial conditions
            let cond_counter: u32 = match condition {
                TerrainStatus::Burning  => 5,
                TerrainStatus::Frozen   => 6,
                TerrainStatus::Shielded => 1,
                _                       => 0,
            };

            let cell = Cell {
                pos: Vector::new(x as f32, y as f32),
                land, condition, cond_counter,
            };

            row.push(cell);
        }
        map[y] = row;
    }
    map
}

/// Cells are the atomic elements that describe what a unit consists of.
/// It holds a position Vector to model. Some conditions are considered temporary in game and
/// will decrement and reset over time
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pos: Vector,
    land: Terrain,
    condition: TerrainStatus,
    cond_counter: u32,
}

impl Cell {
    pub fn new() -> Self {
        Self{
            pos: Vector::new(0.0,0.0),
            land: Terrain::Plain,
            condition: TerrainStatus::Normal,
            cond_counter: 0,
        }
    }
    pub fn get_land(&self) -> Result<&Terrain>       { Ok(&self.land) }
    pub fn get_cond(&self) -> Result<&TerrainStatus> { Ok(&self.condition) }
    pub fn get_counter(&self) -> Result<u32>         { Ok(self.cond_counter) }
    pub fn get_pos(&self) -> Result<Vector>          { Ok(self.pos) }
    //Set func
    pub fn set_land(&mut self, terrain: Terrain)              { self.land = terrain }
    pub fn set_cond(&mut self, terrain_status: TerrainStatus) { self.condition = terrain_status }
    fn set_counter(&mut self, counter: u32)               { self.cond_counter = counter }
    //Decrement cond counter
    ///Decrements condition counter if greater than 0, resets TerrainStatus if counter reaches 0
    pub fn decr_counter(&mut self) {
        if self.cond_counter > 0{
            self.cond_counter -= 1;
            if self.cond_counter <= 0 {
                self.condition = TerrainStatus::Normal;
            }
        }
    }

    /// Increments condition counter if greater than 0 and not Normal
    /// We check for if the counter is greater than 0 because some not-Normal conditions can have 0 counters
    pub fn inc_counter(&mut self) {
        if self.cond_counter > 0 && self.condition != TerrainStatus::Normal {
            self.cond_counter += 1;
        }
    }

    /// Resets land condition without applying effects of it naturally expiring
    pub fn reset_cond(&mut self) {
        self.condition = TerrainStatus::Normal;
        self.set_counter(0);
    }
}

/// The GameBoard is the environment that contains the game data
pub struct GameBoard {
    //Environment
    board: Vec<Vec<Cell>>,
}

impl GameBoard {
    /// Initializes the game state
    pub fn new() -> Result<Self> {
        let level_set = Levels::new().expect("Cannot initialize levels").
            get_level(1).expect("Cannot load level 1");
        let gameboard = generate_map(Vector::new(19.0, 15.0), level_set.0, level_set.1);

        Ok(Self {
            board: gameboard,
        })
    }

    pub fn get_board(&self) -> Result<&Vec<Vec<Cell>>> {
        Ok(&self.board)
    }
    pub fn get_mut_board(&mut self) -> Result<&mut Vec<Vec<Cell>>> {
        Ok(&mut self.board)
    }

    ///Decrements some counters in cell, and resets conditions to Normal if it reaches 0
    /// Sometimes this will change the Terrain tile, such as in the case of TerrainStatus::Burning expiring and a tile becoming blank
    pub fn decrement_temp_cond_counters(&mut self) -> Result<()> {
        for row in &mut self.board {
            for cell in row {
                match cell.get_cond()? {
                    TerrainStatus::Burning    => {
                        if cell.get_counter()? > 0 { cell.set_counter(cell.get_counter()? - 1); }
                        if cell.get_counter()? <= 0 {
                            cell.set_cond(TerrainStatus::Normal);
                            cell.set_land(Terrain::Destroyed)}
                    },
                    TerrainStatus::Frozen     => {
                        if cell.get_counter()? > 0 { cell.set_counter(cell.get_counter()? - 1); }
                        if cell.get_counter()? <= 0 { cell.set_cond(TerrainStatus::Normal); }
                    },
                    _                         => {}
                }
            }
        }

        Ok(())
    }
}