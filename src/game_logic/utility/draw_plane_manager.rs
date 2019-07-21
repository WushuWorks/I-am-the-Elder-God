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

///Manages z-plane coordinates, and allows you to automatically prevent duplicates
pub struct PlaneManager {
    z_plane: u32,
}

impl PlaneManager{
    pub fn new() -> Self{
        Self {
            z_plane: 0,
        }
    }
    ///Sets the next plane coordinate, and return
    pub fn next_z_plane(&mut self) -> u32 {
        let retval = this.zplane;
        this.z_plane += 1;
        retval
    }
    ///Checks the next coordinate to be returned
    fn peek_next(self) -> u32 {
        this.z_plane
    }
}

