use bracket_lib::prelude::*;
use specs::{World, WorldExt};

mod states;
mod systems;
mod ui;
mod mechanisms;
mod maps;
mod players;
mod mobs;

use states::{run_state::RunState, state::State};
use ui::{
    render::Renderable,
    game_log,
};
use mechanisms::field_of_view::FieldOfView;
use maps::{
    blocks::BlocksTile, 
    map::{Map, MapType}, 
    position::Position
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
    create_ecs_components(&mut gs.ecs);

    main_loop(ctx, gs)
}

fn register_ecs_components(ecs: &mut World){
    ecs.register::<Position>();
    ecs.register::<BlocksTile>();
    ecs.register::<FieldOfView>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
    ecs.register::<Mob>();
}

fn create_ecs_components(ecs: &mut World){
    ecs.insert(Map::from_type(MapType::Random));
    ecs.insert(game_log::new());

    create_player(ecs);
    create_mobs(ecs);
}
