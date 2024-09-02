use bracket_lib::prelude::*;
use specs::{Entity, World};
use crate::{TERMINAL_WIDTH, TERMINAL_HEIGHT};
use crate::ui::bottom_bar::BOTTOM_BAR_HEIGHT;
use crate::maps::position::Position;

use super::random_wall_map;

pub const MAP_WIDTH: i32 = TERMINAL_WIDTH;
pub const MAP_HEIGHT: i32 = TERMINAL_HEIGHT-BOTTOM_BAR_HEIGHT-1;

const WALL_GLYPH: char = '~';
const FLOOR_GLYPH: char = '.';

pub enum MapType {
    Random,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Wall, 
    Floor,
}

pub const BLOCKING_TILE_TYPES: [TileType; 1] = [
    TileType::Wall,
];

pub struct Map {
    pub map_type: MapType,
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub discovered_tiles: Vec<bool>,
    pub blocked_tiles: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

impl Map {
    pub fn new(map_type: MapType) -> Map {
        match map_type {
            MapType::Random => random_wall_map::new(), 
        } 
    }

    pub fn populate_blocked_tiles(&mut self){
        let blocking_tiles = BLOCKING_TILE_TYPES.to_vec();

        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked_tiles[i] = blocking_tiles.contains(tile);
        }
    }

    pub fn clear_content_index(&mut self){
        for content in self.tile_content.iter_mut(){
            content.clear();
        }
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm){
    let map = ecs.fetch::<Map>();
    let mut position = Position{ x: 0, y: 0 };

    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.discovered_tiles[idx] {
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
        }
        position.x += 1;
        if position.x > MAP_WIDTH-1 {
            position.x = 0;
            position.y += 1;
        }
    }
}
