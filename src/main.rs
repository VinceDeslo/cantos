use std::cmp::{max, min};
use bracket_lib::prelude::*;
use specs::{Builder, Component, DenseVecStorage, Join, World, WorldExt};
use specs_derive::Component;

const GAME_TITLE: &str = "Cantos";
const TERMINAL_WIDTH: i32 = 80;
const TERMINAL_HEIGHT: i32 = 50;

struct State {
    ecs: World
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component)]
struct Player {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        // Inputs 
        player_input(self, ctx);

        // Automations
        // TBD...

        // Rendering
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph)
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title(GAME_TITLE)
        .build()?;

    let mut gs: State = State {
        ecs: World::new(),
    };

    register_ecs_components(&mut gs.ecs);
    create_player_entity(&mut gs.ecs);

    main_loop(context, gs)
}

fn register_ecs_components(ecs: &mut World){
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();
}

fn create_player_entity(ecs: &mut World){
    ecs.create_entity() 
        .with(Position {
            x: TERMINAL_WIDTH/2, 
            y: TERMINAL_HEIGHT/2
        })
        .with(Renderable {
            glyph: to_cp437('P'),
            fg: RGB::named(SEA_GREEN),
            bg: RGB::named(BLACK)
        })
        .with(Player {})
        .build();
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(TERMINAL_WIDTH-1, max(0, pos.x + delta_x));
        pos.y = min(TERMINAL_HEIGHT-1, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
        None => {} // Nothing happened
    }
}
