use bracket_lib::prelude::*;
use crate::maps::map::{GameMap, MAP_WIDTH, MAP_HEIGHT, TileType};
use crate::maps::position::{START_POSITION, get_position_index};

pub fn new() -> GameMap {
    let map_dimension = (MAP_WIDTH*MAP_HEIGHT) as usize;
    let mut map = GameMap {
        tiles: vec![TileType::Floor; map_dimension],
    };

    // Make external walls
    for x in 0..MAP_WIDTH {
        map.tiles[get_position_index(x, 0)] = TileType::Wall;
        map.tiles[get_position_index(x, MAP_HEIGHT-1)] = TileType::Wall;
    }
    for y in 0..MAP_HEIGHT {
        map.tiles[get_position_index(0, y)] = TileType::Wall;
        map.tiles[get_position_index(MAP_WIDTH-1, y)] = TileType::Wall;
    }

    // Generate random walls
    let mut rng = RandomNumberGenerator::new();
    for _i in 0..300 {
        let x = rng.roll_dice(1, MAP_WIDTH-1);
        let y = rng.roll_dice(1, MAP_HEIGHT-1);
        let idx = get_position_index(x, y);
        if idx != get_position_index(START_POSITION.x, START_POSITION.y) {
            map.tiles[idx] = TileType::Wall;
        }
    }
    
    return map;
}
