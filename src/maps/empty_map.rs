use specs::World;

use crate::maps::map::{Map, TileType};
use crate::maps::position::get_position_index;

use super::builder::MapBuilder;
use super::map::MapType;
use super::position::Position;

pub struct EmptyMapBuilder {
    map: Map,
    start_position: Position,
}

pub fn new_empty_map_builder() -> Box<dyn MapBuilder>{
    return Box::new(EmptyMapBuilder::new());
}

impl MapBuilder for EmptyMapBuilder {
    fn build(&mut self) {
        EmptyMapBuilder::set_type(self);
        EmptyMapBuilder::set_name(self);
        EmptyMapBuilder::set_start(self);
        EmptyMapBuilder::generate_external_walls(self);
    }

    fn get_map(&mut self) -> Map {
        return self.map.clone(); 
    }

    fn get_start(&mut self) -> Position {
        return self.start_position.clone();
    }

    fn spawn_entities(&mut self, _ecs: &mut World) {
       todo!();
    }
}

impl EmptyMapBuilder {
    fn new() -> EmptyMapBuilder {
        EmptyMapBuilder {
            map: Map::new(),
            start_position: Position { x: 0, y: 0 },
        }
    }

    fn set_type(&mut self) {
        self.map.map_type = MapType::Empty;
    }

    fn set_name(&mut self) {
        self.map.name = self.map.map_type.to_string();
    }

    fn set_start(&mut self) {
        self.start_position = Position {
            x: self.map.width / 2,
            y: self.map.height / 2,
        } 
    }

    fn generate_external_walls(&mut self) {
        for x in 0..self.map.width {
            self.map.tiles[get_position_index(x, 0)] = TileType::Wall;
            self.map.tiles[get_position_index(x, self.map.height - 1)] = TileType::Wall;
        }
        for y in 0..self.map.height {
            self.map.tiles[get_position_index(0, y)] = TileType::Wall;
            self.map.tiles[get_position_index(self.map.width - 1, y)] = TileType::Wall;
        }
    }
}
