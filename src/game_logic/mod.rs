/*
Declaring our files as 'pub mod' here allows them to be 'use'd outside of here. In main.rs
for example. Everything that you want to use must have the keyword 'pub', all the way down the function
level.
*/

//This package contains logic that runs the game's core loop

/// Contains logic to run the game state machine (core loop)
pub mod main_state;
/// Contains information about Scenes and their types
pub mod scene_type;
/// Plays music with a system dependant on the fps in draw calls
pub mod music_player;
/// Holds functions to execute draw calls
pub mod draw_helper;