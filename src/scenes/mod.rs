// I am the Elder God. A 3 vs 1 board game made using Quicksilver
//
// Copyright (C) 2019  WushuWorks
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

/*
Declaring our files as 'pub mod' here allows them to be 'use'd outside of here. In main.rs
for example. Everything that you want to use must have the keyword 'pub', all the way down the function
level.
*/

pub mod game;
pub mod intro;
pub mod outro;