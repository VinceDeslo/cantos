use bracket_lib::random::RandomNumberGenerator;

#[derive(Debug)]
pub enum Direction {
    North,
    West,
    South,
    East,
    Static,
}

pub fn get_random_direction() -> Direction {
    let mut rng = RandomNumberGenerator::new();
    let rng_value = rng.roll_dice(1, 5);
    let direction = get_direction(rng_value);
    return direction;
}

fn get_direction(rng_value: i32) -> Direction {
    match rng_value {
        1 => Direction::North,
        2 => Direction::West,
        3 => Direction::South,
        4 => Direction::East,
        _ => Direction::Static,
    }
}
