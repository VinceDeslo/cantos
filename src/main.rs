use bracket_lib::prelude::*;
use specs::{Component, DenseVecStorage, Join, World, WorldExt};
use specs_derive::Component;

mod maps;
mod player;

const GAME_TITLE: &str = "Cantos";

struct State {
    ecs: World
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        // Inputs 
        player::player_input(self, ctx);

        // Automations
        // TBD...

        // Render map
        let map = self.ecs.fetch::<maps::GameMap>();
        maps::draw_map(&map, ctx);

        // Render renderables
        let positions = self.ecs.read_storage::<maps::Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title(GAME_TITLE)
        .build()?;

    let mut gs: State = State {
        ecs: World::new(),
    };

    register_ecs_components(&mut gs.ecs);
    player::create_player_entity(&mut gs.ecs);

    main_loop(ctx, gs)
}

fn register_ecs_components(ecs: &mut World){
    ecs.register::<maps::Position>();
    ecs.register::<Renderable>();
    ecs.register::<player::Player>();

    ecs.insert(maps::new_random_wall_map());
}
