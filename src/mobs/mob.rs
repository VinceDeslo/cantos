use bracket_lib::prelude::*;
use specs::{Builder, Component, DenseVecStorage, World, WorldExt};
use specs_derive::Component;

use crate::{maps::position::Position, Renderable};

// Eventually replace with a raw config file
pub enum MobType {
    Slime,
    Imp,
    Undead,
    Orc,
    Lich,
}

impl MobType {
    pub fn to_string(&self) -> String {
        match self {
            MobType::Slime => "Slime".to_string(),
            MobType::Imp => "Imp".to_string(),
            MobType::Undead => "Undead".to_string(),
            MobType::Orc => "Orc".to_string(),
            MobType::Lich => "Lich".to_string()
        }
    }
    fn glyph(&self) -> FontCharType {
        match self {
            MobType::Slime => to_cp437('S'),
            MobType::Imp => to_cp437('I'),
            MobType::Undead => to_cp437('U'),
            MobType::Orc => to_cp437('O'),
            MobType::Lich => to_cp437('L')
        }
    }
}

#[derive(Component)]
pub struct Mob {
    pub mob_type: MobType,
    pub encountered: bool,
}

pub fn create_mobs(ecs: &mut World) {
    // Implement a mob positioning system
    create_mob_entity(MobType::Slime, 5, 30, ecs);
    create_mob_entity(MobType::Imp, 65, 23, ecs);
    create_mob_entity(MobType::Undead, 42, 25, ecs);
    create_mob_entity(MobType::Orc, 24, 1, ecs);
    create_mob_entity(MobType::Lich, 10, 15, ecs);
}

fn create_mob_entity(mob_type: MobType, x: i32, y: i32, ecs: &mut World){
    ecs.create_entity() 
        .with(Position { x, y })
        .with(Renderable {
            glyph: mob_type.glyph(),
            fg: RGB::named(RED),
            bg: RGB::named(BLACK)
        })
        .with(Mob { 
            mob_type,
            encountered: false,
        })
        .build();
}
