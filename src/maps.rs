use bracket_lib::prelude::*;
use specs::{World, DenseVecStorage, Component};
use specs_derive::Component;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const START_POSITION: Position = Position {
    x: MAP_WIDTH / 2,
    y: MAP_HEIGHT / 2,
};

const WALL_GLYPH: char = '~';
const FLOOR_GLYPH: char = '.';

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, 
    Floor,
}

pub struct GameMap {
    tiles: Vec<TileType>,
} 

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub fn draw_map(map: &GameMap, ctx: &mut BTerm){
    let mut position = Position{ x: 0, y: 0 };

    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    position.x, 
                    position.y, 
                    RGB::named(DARKGRAY), 
                    RGB::named(BLACK), 
                    to_cp437(FLOOR_GLYPH),
                )
            }, 
            TileType::Wall => {
                ctx.set(
                    position.x, 
                    position.y, 
                    RGB::named(WHITE), 
                    RGB::named(BLACK), 
                    to_cp437(WALL_GLYPH),
                )
            }
        }
        position.x += 1;
        if position.x > MAP_WIDTH-1 {
            position.x = 0;
            position.y += 1;
        }
    }
}

pub fn new_random_wall_map() -> GameMap {
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

fn get_position_index(x: i32, y: i32) -> usize {
    (y * MAP_WIDTH) as usize + x as usize
}

pub fn not_wall_collision(destination_x: i32, destination_y: i32, ecs: &World) -> bool {
    let map = ecs.fetch::<GameMap>();

    let destination_idx = get_position_index(destination_x, destination_y);

    return map.tiles[destination_idx] != TileType::Wall;
}
