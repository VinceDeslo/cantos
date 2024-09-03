use specs::World;

use super::{map::Map, position::Position};

pub trait MapBuilder {
    fn build(&mut self); 
    fn get_map(&mut self) -> Map;
    fn get_start(&mut self) -> Position;
    fn spawn_entities(&mut self, ecs: &mut World); 
}
