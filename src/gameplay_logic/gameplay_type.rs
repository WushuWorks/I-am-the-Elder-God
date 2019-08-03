/*
This defines the various enum elements that are a part of gameplay data
*/

///The types of lands
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(unused)]
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

impl Terrain{
    ///Gets the Atlas key that maps to the value
    pub fn key(&self) -> Option<&str> {
        match self {
            Terrain::Road       => Some("R"),
            Terrain::Plain      => Some("#"),
            Terrain::Forest     => Some("F"),
            Terrain::Mountain   => Some("M"),
            Terrain::City       => Some("C"),
            Terrain::Wall       => Some("W"),
            Terrain::Pit        => Some("X"),
            Terrain::Empty      => Some("-"),
        }
    }
}

/// Translates a key string to a Terrain enum if possible, returns None if not found
pub fn to_terrain( key: &str) -> Option<Terrain> {
    match key {
        "R"     => Some(Terrain::Road),
        "#"     => Some(Terrain::Plain),
        "F"     => Some(Terrain::Forest),
        "M"     => Some(Terrain::Mountain),
        "C"     => Some(Terrain::City),
        "W"     => Some(Terrain::Wall),
        "X"     => Some(Terrain::Pit),
        "-"     => Some(Terrain::Empty),
        _       => None,
    }
}
///The conditions a land can be in
#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(unused)]
pub enum TerrainStatus {
    Normal,
    Burning,
    Frozen,
    Shielded,
    Impassable,
}

impl TerrainStatus {
    ///Translates a TerrainStatus enum to a str key, returns None if not found (this should never happen though)
    pub fn key(&self) -> Option<&str> {
        match self {
            TerrainStatus::Normal     => Some("N"),
            TerrainStatus::Burning    => Some("B"),
            TerrainStatus::Frozen     => Some("F"),
            TerrainStatus::Shielded   => Some("S"),
            TerrainStatus::Impassable => Some("-"),
        }
    }
}

/// Translates a key string to a TerrainStatus enum if possible, returns None if not found
pub fn to_condition(key: &str) -> Option<TerrainStatus> {
    match key {
        "N"     => Some(TerrainStatus::Normal),
        "B"     => Some(TerrainStatus::Burning),
        "F"     => Some(TerrainStatus::Frozen),
        "S"     => Some(TerrainStatus::Shielded),
        "-"     => Some(TerrainStatus::Impassable),
        _       => None,
    }
}