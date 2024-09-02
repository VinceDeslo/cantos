use specs::{Component, DenseVecStorage};
use specs_derive::Component;

use crate::maps::map::{MAP_WIDTH, MAP_HEIGHT};

pub const START_POSITION: Position = Position {
    x: MAP_WIDTH / 2,
    y: MAP_HEIGHT / 2,
};

#[derive(Component, PartialEq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn get_position_index(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH) as usize + x as usize
}
