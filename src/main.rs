use bracket_lib::prelude::*;
use specs::{World, WorldExt};

mod states;
mod systems;
mod ui;
mod maps;
mod players;
mod mobs;

use states::{run_state::RunState, state::State};

use ui::{
    render::Renderable,
    game_log,
};
use maps::{
    position::Position, 
    random_wall_map,
};

use players::player::{Player, create_player};

use mobs::mob::{create_mobs, Mob};

const GAME_TITLE: &str = "Cantos";
const TERMINAL_WIDTH: i32 = 80;
const TERMINAL_HEIGHT: i32 = 50;

fn main() -> BError {
    let mut ctx = BTermBuilder::simple80x50()
        .with_title(GAME_TITLE)
        .build()?;

    ctx.with_post_scanlines(true);

    let mut gs: State = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };

    register_ecs_components(&mut gs.ecs);
    create_player(&mut gs.ecs);
    create_mobs(&mut gs.ecs);

    main_loop(ctx, gs)
}

fn register_ecs_components(ecs: &mut World){
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Mob>();

    ecs.insert(random_wall_map::new());
    ecs.insert(game_log::new());
}
