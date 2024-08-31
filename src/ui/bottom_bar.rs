use bracket_lib::prelude::*;
use specs::prelude::*;

use crate::{TERMINAL_WIDTH, TERMINAL_HEIGHT};
use crate::ui::game_log::GameLog;

pub const BOTTOM_BAR_HEIGHT: i32 = 10;

pub fn draw_bottom_bar(ecs: &World, ctx : &mut BTerm) {
    ctx.draw_box(
        0, 
        TERMINAL_HEIGHT - BOTTOM_BAR_HEIGHT - 1,
        TERMINAL_WIDTH - 1, 
        BOTTOM_BAR_HEIGHT, 
        RGB::named(WHITE), 
        RGB::named(BLACK),
    );

    let log = ecs.fetch::<GameLog>();
    
    let mut y = TERMINAL_HEIGHT - BOTTOM_BAR_HEIGHT + 1;
    for s in log.entries.iter().rev() {
        if y < TERMINAL_HEIGHT - 1 {
            ctx.print(2, y, s); 
        }
        y += 1;
    }
}
