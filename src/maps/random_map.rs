use bracket_lib::prelude::*;
use specs::World;
use crate::maps::map::{Map, TileType};
use crate::maps::position::get_position_index;

use super::builder::MapBuilder;
use super::map::MapType;
use super::position::Position;

const RANDOM_RATIO: usize = 300;

pub struct RandomMapBuilder {
    map: Map,
    start_position: Position,
}

pub fn new_random_map_builder() -> Box<dyn MapBuilder>{
    return Box::new(RandomMapBuilder::new());
}

impl MapBuilder for RandomMapBuilder {
    fn build(&mut self) {
        RandomMapBuilder::set_type(self);
        RandomMapBuilder::set_name(self);
        RandomMapBuilder::set_start(self);
        RandomMapBuilder::generate_external_walls(self);
        RandomMapBuilder::generate_random_walls(self);
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

impl RandomMapBuilder {
    fn new() -> RandomMapBuilder {
        RandomMapBuilder {
            map: Map::new(),
            start_position: Position { x: 0, y: 0 },
        }
    }

    fn set_type(&mut self) {
        self.map.map_type = MapType::Random;
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

    fn generate_random_walls(&mut self) {
        let start = self.get_start();
        let mut rng = RandomNumberGenerator::new();

        for _i in 0..RANDOM_RATIO {
            let x = rng.roll_dice(1, self.map.width - 1);
            let y = rng.roll_dice(1, self.map.height - 1);
            let idx = get_position_index(x, y);

            if idx != get_position_index(start.x, start.y) {
                self.map.tiles[idx] = TileType::Wall;
            }
        }
    }
}
