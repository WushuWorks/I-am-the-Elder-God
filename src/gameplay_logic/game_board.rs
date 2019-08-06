/*
This is the environment layer, which models the board and its contents
*/

use quicksilver::prelude::*;
use crate::gameplay_logic::gameplay_type::{Terrain, TerrainStatus, to_condition, to_terrain};
use crate::gameplay_logic::game_levels::Levels;
use crate::gameplay_logic::entities::{Entity, PlayerType};

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

            let placeholder = Entity::new_npc(PlayerType::Undetermined, 0,0,0,0,
                                              Vector::new(x as f32,y as f32), true, false)
                                                   .expect("Cannot allocate placeholder NPC in Cell::new.");

            let cell = Cell {
                pos: Vector::new(x as f32, y as f32),
                land: to_terrain(tile_key).expect("Failed to translate key to Terrain game_board::generate_map"),
                condition: to_condition(tile_cond_key).expect("Failed to translate key to TerrainStatus game_board::generate_map"),
                occupant: placeholder,
            };

            row.push(cell);
        }
        map[y] = row;
    }
    map
}

/// Cells are the atomic elements that describe what a unit consists of.
/// It holds a position Vector to model
#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pos: Vector,
    land: Terrain,
    condition: TerrainStatus,
    occupant: Entity,
}

#[allow(unused)]
impl Cell {
    pub fn new() -> Self {
        let placeholder = Entity::new_npc(PlayerType::Undetermined, 0,0,0,0,
                                           Vector::new(0.0,0.0), true, false)
                                                .expect("Cannot allocate placeholder NPC in Cell::new.");
        Self{
            pos: Vector::new(0.0,0.0),
            land: Terrain::Plain,
            condition: TerrainStatus::Normal,
            occupant: placeholder,
        }
    }
    pub fn get_land(&self) -> Result<&Terrain>       { Ok(&self.land) }
    pub fn get_cond(&self) -> Result<&TerrainStatus> { Ok(&self.condition) }
    pub fn get_pos(&self) -> Result<Vector>         { Ok(self.pos) }
    pub fn get_occupant(&self) -> Result<&Entity>    { Ok(&self.occupant) }
    ///Swaps the passed current entity for the passed entity
    pub fn new_entity(&mut self, new_entity: Entity) -> Result<()>   {
        self.occupant = new_entity;
        Ok(())
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
}