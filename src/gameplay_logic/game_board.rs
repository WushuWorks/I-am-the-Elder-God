/*
This is the environment layer, which models the board that the other layers sit on top of
and supports logic to render its own state
*/

use quicksilver::prelude::*;
use crate::gameplay_logic::gameplay_type::{Terrain, TerrainStatus};

/// Generation logic adapted from [Quicksilver Rougelike](https://github.com/tomassedovic/quicksilver-roguelike)
/// Generates a map with the initial game state
/// Creates map by defining a Normal Plain and then modifying it to the end state
fn generate_map(size: Vector) -> Vec<Cell> {
    let width = size.x as usize;
    let height = size.y as usize;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut cell = Cell {
                pos: Vector::new(x as f32, y as f32),
                land: Terrain::Plain,
                condition: TerrainStatus::Normal,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                cell.land = Terrain::Wall;
                cell.condition = TerrainStatus::Impassable;
            };
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
}

/// The GameBoard is the environment that contains a 2d array of Cells.
pub struct GameBoard {
    board: Vec<Cell>,
}

impl GameBoard {
    /// Initializes the game state
    pub fn new() -> Result<Self> {
        Ok(Self{
            board: generate_map(Vector::new(13.0, 13.0))
        })
    }

    pub fn get_board(&self) ->Result<&Vec<Cell>>{
        Ok(&self.board)
    }
}