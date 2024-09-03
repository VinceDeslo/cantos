use specs::{Component, DenseVecStorage};
use specs_derive::Component;

use crate::maps::map::MAP_WIDTH;

#[derive(Component, PartialEq, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn get_position_index(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH) as usize + x as usize
}
