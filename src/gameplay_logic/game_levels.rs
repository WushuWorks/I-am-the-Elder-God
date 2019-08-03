/*
This is a struct to hold level vectors to avoid complicated rendering logic
*/

use quicksilver::prelude::Result;

///Defines a set of "-"9x"-"5 levels, by convention only the circle within the center "-"3x"-"3 square are
/// changed to anything besides the null space
#[derive(Debug, PartialEq, Clone)]
pub struct Levels {
    level1: Vec<String>,
    level1_conditions: Vec<String>,
}

/// Defines a macro to make a Vec of String
/// taken from https://stackoverflow.com/questions/38183551/concisely-initializing-a-vector-of-strings
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}


impl Levels{
    ///Creates the levels and conditions for the levels
    pub fn new() -> Result<Self> {
        //This is mirrored across the xy axis
        //Done like this to avoid confusing rendering logic
        Ok(Self {
            level1: vec_of_strings![
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "#", "#", "#", "#", "#", "-", "-", "-", "-", "-",
                "-", "-", "-", "#", "#", "#", "#", "#", "#", "#", "#", "#", "-", "-", "-",
                "-", "-", "#", "#", "#", "#", "#", "#", "#", "M", "M", "#", "#", "-", "-",
                "-", "-", "#", "#", "M", "#", "#", "#", "#", "M", "M", "#", "#", "-", "-",
                "-", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "-",
                "-", "#", "#", "#", "#", "#", "#", "C", "#", "#", "M", "M", "M", "#", "-",
                "-", "#", "#", "M", "#", "#", "C", "C", "C", "#", "M", "M", "M", "#", "-",
                "-", "#", "#", "#", "#", "#", "#", "C", "#", "#", "M", "M", "M", "#", "-",
                "-", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "#", "-",
                "-", "-", "#", "#", "M", "#", "#", "#", "#", "#", "#", "#", "#", "-", "-",
                "-", "-", "#", "#", "#", "#", "#", "#", "#", "M", "#", "#", "#", "-", "-",
                "-", "-", "-", "#", "#", "#", "#", "#", "#", "#", "#", "#", "-", "-", "-",
                "-", "-", "-", "-", "-", "#", "#", "#", "#", "#", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-"
            ],
            level1_conditions: vec_of_strings![
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "N", "N", "N", "N", "N", "-", "-", "-", "-", "-",
                "-", "-", "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-", "-", "-",
                "-", "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-", "-",
                "-", "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-", "-",
                "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-",
                "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-",
                "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-",
                "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-",
                "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-",
                "-", "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-", "-",
                "-", "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-", "-",
                "-", "-", "-", "N", "N", "N", "N", "N", "N", "N", "N", "N", "-", "-", "-",
                "-", "-", "-", "-", "-", "N", "N", "N", "N", "N", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-",
                "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-", "-"
            ],
        })
    }

    ///Return a tuple clone of level icon data and condition data in
    /// the format below. Both tuples are Vec<String>. If not found returns None.
    ///
    /// ```
    /// Some((level_data, level_data_conditions))
    /// ```
    pub fn get_level(&self, level: usize) -> Option<(Vec<String>, Vec<String>)>{
        match level{
            1 => Some((self.level1.clone(), self.level1_conditions.clone())),
            _ => None,
        }
    }
}