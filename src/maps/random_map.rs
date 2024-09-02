use bracket_lib::prelude::*;
use crate::maps::map::{Map, TileType};
use crate::maps::position::get_position_index;

use super::builder::MapBuilder;
use super::position::Position;

const RANDOM_RATIO: usize = 300;

pub struct RandomMapBuilder {}

impl MapBuilder for RandomMapBuilder {
    fn build() -> Map {
        let mut map = Map::new();
        RandomMapBuilder::generate_external_walls(&mut map);
        RandomMapBuilder::generate_random_walls(&mut map);
        return map
    }
}

impl RandomMapBuilder {
    fn get_start_position(map: &Map) -> Position {
        Position {
            x: map.width / 2,
            y: map.height / 2,
        } 
    }

    fn generate_external_walls(map: &mut Map) {
        for x in 0..map.width {
            map.tiles[get_position_index(x, 0)] = TileType::Wall;
            map.tiles[get_position_index(x, map.height - 1)] = TileType::Wall;
        }
        for y in 0..map.height {
            map.tiles[get_position_index(0, y)] = TileType::Wall;
            map.tiles[get_position_index(map.width - 1, y)] = TileType::Wall;
        }
    }

    fn generate_random_walls(map: &mut Map) {
        let start = RandomMapBuilder::get_start_position(map);
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..RANDOM_RATIO {
            let x = rng.roll_dice(1, map.width - 1);
            let y = rng.roll_dice(1, map.height - 1);
            let idx = get_position_index(x, y);

            if idx != get_position_index(start.x, start.y) {
                map.tiles[idx] = TileType::Wall;
            }
        }
    }
}
