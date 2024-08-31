use std::{cmp::{max, min}, usize};
use bracket_lib::prelude::*;
use specs::{Builder, Component, DenseVecStorage, Join, World, WorldExt};
use specs_derive::Component;

const GAME_TITLE: &str = "Cantos";
const TERMINAL_WIDTH: i32 = 80;
const TERMINAL_HEIGHT: i32 = 50;
const START_POSITION: Position = Position {
    x: TERMINAL_WIDTH / 2,
    y: TERMINAL_HEIGHT / 2,
};
const PLAYER_GLYPH: char = 'O';
const WALL_GLYPH: char = '~';
const FLOOR_GLYPH: char = '.';

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

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall, 
    Floor,
}

struct GameMap {
    tiles: Vec<TileType>,
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

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title(GAME_TITLE)
        .build()?;

    let mut gs: State = State {
        ecs: World::new(),
    };

    register_ecs_components(&mut gs.ecs);
    create_player_entity(&mut gs.ecs);

    main_loop(ctx, gs)
}

fn register_ecs_components(ecs: &mut World){
    ecs.register::<Position>();
    ecs.register::<Renderable>();
    ecs.register::<Player>();

    ecs.insert(new_random_wall_map());
}

fn create_player_entity(ecs: &mut World){
    ecs.create_entity() 
        .with(START_POSITION)
        .with(Renderable {
            glyph: to_cp437(PLAYER_GLYPH),
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
        let destination_x = pos.x + delta_x;
        let destination_y = pos.y + delta_y;

        if not_wall_collision(destination_x, destination_y, ecs) {
            pos.x = min(TERMINAL_WIDTH-1, max(0, destination_x));
            pos.y = min(TERMINAL_HEIGHT-1, max(0, destination_y));
        }
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

fn get_position_index(x: i32, y: i32) -> usize {
    (y * TERMINAL_WIDTH) as usize + x as usize
}

fn new_random_wall_map() -> GameMap {
    let map_dimension = (TERMINAL_WIDTH*TERMINAL_HEIGHT) as usize;
    let mut map = GameMap {
        tiles: vec![TileType::Floor; map_dimension],
    };

    // Make external walls
    for x in 0..TERMINAL_WIDTH {
        map.tiles[get_position_index(x, 0)] = TileType::Wall;
        map.tiles[get_position_index(x, TERMINAL_HEIGHT-1)] = TileType::Wall;
    }
    for y in 0..TERMINAL_HEIGHT {
        map.tiles[get_position_index(0, y)] = TileType::Wall;
        map.tiles[get_position_index(TERMINAL_WIDTH-1, y)] = TileType::Wall;
    }

    // Generate random walls
    let mut rng = RandomNumberGenerator::new();
    for _i in 0..500 {
        let x = rng.roll_dice(1, TERMINAL_WIDTH-1);
        let y = rng.roll_dice(1, TERMINAL_HEIGHT-1);
        let idx = get_position_index(x, y);
        if idx != get_position_index(START_POSITION.x, START_POSITION.y) {
            map.tiles[idx] = TileType::Wall;
        }
    }
    
    return map;
}

fn draw_map(map: &GameMap, ctx: &mut BTerm){
    let mut position = Position{ x: 0, y: 0 };

    for tile in map.tiles.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    position.x, 
                    position.y, 
                    RGB::named(GREY), 
                    RGB::named(BLACK), 
                    to_cp437(FLOOR_GLYPH),
                )
            }, 
            TileType::Wall => {
                ctx.set(
                    position.x, 
                    position.y, 
                    RGB::named(WHITE), 
                    RGB::named(BLACK), 
                    to_cp437(WALL_GLYPH),
                )
            }
        }
        position.x += 1;
        if position.x > TERMINAL_WIDTH-1 {
            position.x = 0;
            position.y += 1;
        }
    }
}

fn not_wall_collision(destination_x: i32, destination_y: i32, ecs: &World) -> bool {
    let map = ecs.fetch::<GameMap>();

    let destination_idx = get_position_index(destination_x, destination_y);

    return map.tiles[destination_idx] != TileType::Wall;
}
