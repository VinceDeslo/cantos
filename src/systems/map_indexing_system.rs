use specs::prelude::*;

use crate::maps::{
    blocks::BlocksTile, 
    map::Map, 
    position::{get_position_index, Position}
};

pub struct MapIndexingSystem {}

impl<'a> System<'a> for MapIndexingSystem {
    type SystemData = ( 
        WriteExpect<'a, Map>,
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, position, blockers) = data;

        map.populate_blocked_tiles();
        map.clear_content_index();
        for (entity, position) in (&entities, &position).join() {
            let idx = get_position_index(position.x, position.y);

            // If the entity blocks, update block list
            let _b = blockers.get(entity);
            if let Some(_b) = _b {
                map.blocked_tiles[idx] = true;
            }

            // Set entity in index slot
            map.tile_content[idx].push(entity);
        }
    }
}
