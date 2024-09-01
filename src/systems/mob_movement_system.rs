use specs::prelude::*; 
use crate::{
    maps::{
        position::Position,
        direction::{Direction, get_random_direction},
    },
    mobs::mob::Mob, 
};

pub struct MobMovementSystem {}

impl<'a> System<'a> for MobMovementSystem {
    type SystemData = ( 
        WriteStorage<'a, Position>,
        ReadStorage<'a, Mob>,
    );

    fn run(&mut self, data: Self::SystemData) {
        random_movement(data);
    }
}

fn random_movement(data: <MobMovementSystem as specs::System>::SystemData){
    let(mut position, mob) = data;
    
    for (mob_position, _mob) in (&mut position, &mob).join() {
        let direction = get_random_direction();

        match direction {
            Direction::North => mob_position.y += 1,
            Direction::West=> mob_position.x += 1,
            Direction::South => mob_position.y -= 1,
            Direction::East => mob_position.x -= 1,
            Direction::Static => {},
        };
    }
}
