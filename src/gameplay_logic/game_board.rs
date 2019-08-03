/*
This is the environment layer, which models the board that the other layers sit on top of
and supports logic to render its own state
*/

use quicksilver::prelude::*;
use crate::gameplay_logic::gameplay_type::{Terrain, TerrainStatus, to_condition, to_terrain};
use crate::gameplay_logic::game_levels::Levels;

/// Generation logic adapted from [Quicksilver Rougelike](https://github.com/tomassedovic/quicksilver-roguelike)
/// Generates a map with the initial game state
/// Creates map by defining a Normal Plain and then modifying it to the end state
fn generate_map(size: Vector, level: Vec<String>, level_condition: Vec<String>) -> Vec<Cell> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut loaded_level = level.iter();
    let mut loaded_level_conditions = level_condition.iter();
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut cell = Cell {
                pos: Vector::new(x as f32, y as f32),
                land: Terrain::Plain,
                condition: TerrainStatus::Normal,
            };

            let tile_key = loaded_level.next().expect("Tried to allocate wrong sized level");
            let tile_cond_key = loaded_level_conditions.next().expect("Tried to allocate wrong sized level");

            cell.land = to_terrain(tile_key).expect("Failed to translate key to Terrain game_board::generate_map");
            cell.condition = to_condition(tile_cond_key).expect("Failed to translate key to TerrainStatus game_board::generate_map");


            map.push(cell);
        }
    }
    map
}

///Cells are the atomic elements that describe what a unit consists of.
pub struct Cell {
    pos: Vector,
    land: Terrain,
    condition: TerrainStatus,
}

impl Cell{
    pub fn get_land(&self) -> Result<&Terrain>{
        Ok(&self.land)
    }
    pub fn get_pos(&self) -> Result<&Vector> { Ok(&self.pos) }
}

/// The GameBoard is the environment that contains a 2d array of Cells.
pub struct GameBoard {
    board: Vec<Cell>,
}

impl GameBoard {
    /// Initializes the game state
    pub fn new() -> Result<Self> {
        let level_set = Levels::new().expect("Cannot initialize levels").
                                                   get_level(1).expect("Cannot load level 1");

        Ok(Self{
            board: generate_map(Vector::new(19.0, 15.0), level_set.0, level_set.1),
        })
    }

    pub fn get_board(&self) ->Result<&Vec<Cell>>{
        Ok(&self.board)
    }
}