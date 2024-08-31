use bracket_lib::prelude::*;
use crate::maps::position::Position;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

const WALL_GLYPH: char = '~';
const FLOOR_GLYPH: char = '.';

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall, 
    Floor,
}

pub struct GameMap {
    pub tiles: Vec<TileType>,
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
