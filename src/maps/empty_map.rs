use crate::maps::map::{Map, TileType};
use crate::maps::position::get_position_index;

use super::builder::MapBuilder;
use super::position::Position;

pub struct EmptyMapBuilder {}

impl MapBuilder for EmptyMapBuilder {
    fn build() -> Map {
        let mut map = Map::new();
        EmptyMapBuilder::generate_external_walls(&mut map);
        return map
    }
}

impl EmptyMapBuilder {
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
}
