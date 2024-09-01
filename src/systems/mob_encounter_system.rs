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
            if mob_position == player_position && !mob.encountered {
                mob.encountered = true;
                let encounter_msg = format!("Encountered a {}", mob.mob_type.to_string());
                console::log(encounter_msg);
            }
        }
    }
}
