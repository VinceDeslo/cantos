use std::cmp::{max, min};
use bracket_lib::prelude::*;
use specs::{Builder, Component, DenseVecStorage, World, WorldExt, Join};
use specs_derive::Component;

use crate::{Renderable, State};
use crate::maps;

const PLAYER_GLYPH: char = 'O';

#[derive(Component)]
pub struct Player {}

pub fn create_player_entity(ecs: &mut World){
    ecs.create_entity() 
        .with(maps::START_POSITION)
        .with(Renderable {
            glyph: to_cp437(PLAYER_GLYPH),
            fg: RGB::named(MAGENTA),
            bg: RGB::named(BLACK)
        })
        .with(Player {})
        .build();
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
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

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<maps::Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_x = pos.x + delta_x;
        let destination_y = pos.y + delta_y;

        if maps::not_wall_collision(destination_x, destination_y, ecs) {
            pos.x = min(maps::MAP_WIDTH-1, max(0, destination_x));
            pos.y = min(maps::MAP_HEIGHT-1, max(0, destination_y));
        }
    }
}
