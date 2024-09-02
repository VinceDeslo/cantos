use bracket_lib::prelude::*;
use specs::prelude::*; 
use crate::{
    maps::position::Position,
    mobs::mob::Mob, 
    players::player::Player,
};

pub struct MobEncounterSystem {}

impl<'a> System<'a> for MobEncounterSystem {
    type SystemData = ( 
        ReadStorage<'a, Position>,
        WriteStorage<'a, Mob>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        detect_encounter(data);
    }
}

fn detect_encounter(data: <MobEncounterSystem as specs::System>::SystemData){
    let(position, mut mob, player) = data;

    for (mob_position, mob) in (&position, &mut mob).join() {
        for (player_position, _player) in (&position, &player).join() {
            let distance = DistanceAlg::Pythagoras.distance2d(
                Point::new(player_position.x, player_position.y),
                Point::new(mob_position.x, mob_position.y),
            );

            if distance < 1.5 && !mob.encountered {
                mob.encountered = true;
                println!(
                    "Encountered {} {}",
                    mob.mob_type.get_article(),
                    mob.mob_type.to_string()
                );
            }
        }
    }
}
