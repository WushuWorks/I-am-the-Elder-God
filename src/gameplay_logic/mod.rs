/*
Declaring our files as 'pub mod' here allows them to be 'use'd outside of here. In main.rs
for example. Everything that you want to use must have the keyword 'pub', all the way down the function
level.
*/

//This package contains logic to implement gameplay

///Logic that manages the games board
pub mod game_board;
pub mod gameplay_type;