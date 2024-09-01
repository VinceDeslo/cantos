use bracket_lib::prelude::Point;
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

#[derive(Component)]
pub struct FieldOfView {
    pub visible_tiles : Vec<Point>,
    pub range : i32
}
