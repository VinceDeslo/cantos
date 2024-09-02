use std::cmp::{max, min};
use specs::{prelude::*, shred::PanicHandler}; 
use crate::{
    maps::{
        direction::{get_random_direction, Direction}, 
        map::{Map, MAP_HEIGHT, MAP_WIDTH}, 
        position::{get_position_index, Position}
    },
    mobs::mob::Mob, 
};

pub struct MobMovementSystem {}

impl<'a> System<'a> for MobMovementSystem {
    type SystemData = ( 
        ReadExpect<'a, Map>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Mob>,
    );

    fn run(&mut self, data: Self::SystemData) {
        random_movement(data);
    }
}

fn random_movement(data: <MobMovementSystem as specs::System>::SystemData){
    let(map, mut position, mob) = data;
    
    for (mob_position, _mob) in (&mut position, &mob).join() {
        let direction = get_random_direction();

        match direction {
            Direction::North => try_move_mob(0, 1, mob_position, &map),
            Direction::West => try_move_mob(1, 0, mob_position, &map),
            Direction::South => try_move_mob(0, -1, mob_position, &map),
            Direction::East => try_move_mob(-1, 0, mob_position, &map),
            Direction::Static => {},
        };
    }
}

fn try_move_mob(
    delta_x: i32, 
    delta_y: i32, 
    position: &mut Position, 
    map: &Read<'_, Map, PanicHandler>
) {
    let destination_x = position.x + delta_x;
    let destination_y = position.y + delta_y;

    let bound_x = min(MAP_WIDTH-1, max(0, destination_x));
    let bound_y = min(MAP_HEIGHT-1, max(0, destination_y));

    let destination_idx = get_position_index(bound_x, bound_y);

    if !map.blocked_tiles[destination_idx] {
        position.x = bound_x;
        position.y = bound_y;
    }
}
