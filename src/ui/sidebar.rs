use bracket_lib::prelude::*;
use specs::World;

use crate::{maps::map::Map, TERMINAL_HEIGHT, TERMINAL_WIDTH};

use super::bottom_bar::BOTTOM_BAR_HEIGHT;

pub const SIDEBAR_WIDTH: i32 = 30;
pub const SIDEBAR_HEIGHT: i32 = TERMINAL_HEIGHT - BOTTOM_BAR_HEIGHT;

pub fn draw_sidebar(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(
        TERMINAL_WIDTH - SIDEBAR_WIDTH - 1,
        0,
        SIDEBAR_WIDTH,
        SIDEBAR_HEIGHT - 1,
        RGB::named(WHITE), 
        RGB::named(BLACK),
    );

    draw_map_name(ecs, ctx);
}

fn draw_map_name(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();
    ctx.print_color(
        TERMINAL_WIDTH - (SIDEBAR_WIDTH - 1), 
        2, 
        WHITE, 
        BLACK, 
        &map.name
    );
}
