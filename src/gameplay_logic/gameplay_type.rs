/*
This defines the various enum elements that are a part of gameplay data
*/

///The types of lands
pub enum Terrain {
    Road,
    Plain,
    Forest,
    Mountain,
    City,
    Wall,
    Pit,
    Empty, //Define invisible space with this
}

///The conditions a land can be in
pub enum TerrainStatus {
    Normal,
    Burning,
    Frozen,
    Shielded,
    Impassable,
}