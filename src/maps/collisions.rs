use specs::World;
use crate::maps::map::{GameMap, TileType};
use crate::maps::position::get_position_index;

pub fn not_wall_collision(destination_x: i32, destination_y: i32, ecs: &World) -> bool {
    let map = ecs.fetch::<GameMap>();

    let destination_idx = get_position_index(destination_x, destination_y);

    return map.tiles[destination_idx] != TileType::Wall;
}
