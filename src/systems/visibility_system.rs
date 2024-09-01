use bracket_lib::prelude::{field_of_view, Point};
use specs::prelude::*; 
use crate::{
    maps::{map::Map, position::{get_position_index, Position}}, 
    mechanisms::field_of_view::FieldOfView, 
    players::player::Player,
};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = ( 
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, FieldOfView>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>
    );

    fn run(&mut self, data: Self::SystemData) {
        run_visibility(data);
    }
}

fn run_visibility(data: <VisibilitySystem as specs::System>::SystemData){
    let(mut map, entities, mut fov, position, player) = data;

    for (entities, fov, position) in (&entities, &mut fov, &position).join() {
        let point = Point::new(position.x, position.y); 
        fov.visible_tiles.clear();
        fov.visible_tiles = field_of_view(point, fov.range, &*map);
        fov.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height );

        // If this is the player, reveal what they can see
        let p : Option<&Player> = player.get(entities);
        if let Some(p) = p {
            for vision in fov.visible_tiles.iter() {
                let idx = get_position_index(vision.x, vision.y);
                map.visible_tiles[idx] = true;
            }
        }
    }
}
