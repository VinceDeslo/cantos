use std::cmp::{max, min};
use bracket_lib::prelude::*;
use specs::{Builder, Component, DenseVecStorage, World, WorldExt, Join};
use specs_derive::Component;

use crate::maps::blocks::BlocksTile;
use crate::maps::position::get_position_index;
use crate::mechanisms::field_of_view::FieldOfView;
use crate::states::run_state::RunState;
use crate::{Renderable, State};
use crate::maps::{
    map::{Map, MAP_WIDTH, MAP_HEIGHT},
    position::Position,
};

const PLAYER_GLYPH: char = '@';
const PLAYER_FOV: i32 = 5; 

#[derive(Component)]
pub struct Player {}

pub fn create_player(ecs: &mut World, position: Position){
    ecs.create_entity() 
        .with(position)
        .with(Renderable {
            glyph: to_cp437(PLAYER_GLYPH),
            fg: RGB::named(GREEN),
            bg: RGB::named(BLACK)
        })
        .with(Player {})
        .with(FieldOfView {
            visible_tiles: Vec::new(),
            range: PLAYER_FOV,
        })
        .with(BlocksTile {})
        .build();
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => { return RunState::Paused }
        },
        None => { return RunState::Paused }
    }
    return RunState::Running;
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, position) in (&mut players, &mut positions).join() {
        let destination_x = position.x + delta_x;
        let destination_y = position.y + delta_y;

        let bound_x = min(MAP_WIDTH-1, max(0, destination_x));
        let bound_y = min(MAP_HEIGHT-1, max(0, destination_y));

        let destination_idx = get_position_index(bound_x, bound_y);

        if !map.blocked_tiles[destination_idx] {
            position.x = bound_x;
            position.y = bound_y;
        }
    }
}
