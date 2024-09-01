use bracket_lib::prelude::*;
use specs::{World, WorldExt, Join, RunNow};

use crate::Renderable;
use crate::ui::bottom_bar::draw_bottom_bar;
use crate::maps::{
    map::{GameMap, draw_map}, 
    position::Position, 
};

use crate::players::player::player_input;

use crate::mobs::{
    mob_encounter_system::MobEncounterSystem,
    mob_movement_system::MobMovementSystem
};

pub struct State {
    pub ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut mob_movements = MobMovementSystem{};
        mob_movements.run_now(&self.ecs);

        let mut mob_encounters = MobEncounterSystem{};
        mob_encounters.run_now(&self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        self.run_systems();

        // Inputs 
        player_input(self, ctx);

        // Automations
        // TBD...
        
        // Render interface
        draw_bottom_bar(&self.ecs, ctx);

        // Render map
        let map = self.ecs.fetch::<GameMap>();
        draw_map(&map, ctx);

        // Render renderables
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}
