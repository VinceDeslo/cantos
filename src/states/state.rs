use bracket_lib::prelude::*;
use specs::{World, WorldExt, Join, RunNow};

use crate::Renderable;
use crate::ui::bottom_bar::draw_bottom_bar;
use crate::maps::{
    map::draw_map, 
    position::Position, 
};

use crate::players::player::player_input;

use crate::systems::{
    visibility_system::VisibilitySystem,
    mob_encounter_system::MobEncounterSystem,
    mob_movement_system::MobMovementSystem
};

use crate::states::run_state::RunState;

pub struct State {
    pub ecs: World,
    pub run_state: RunState
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility = VisibilitySystem{};
        visibility.run_now(&self.ecs);

        let mut mob_movements = MobMovementSystem{};
        mob_movements.run_now(&self.ecs);

        let mut mob_encounters = MobEncounterSystem{};
        mob_encounters.run_now(&self.ecs);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }
        
        draw_bottom_bar(&self.ecs, ctx);
        draw_map(&self.ecs, ctx);

        // Render renderables
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}
