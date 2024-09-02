pub struct GameLog {
    pub entries: Vec<String>,
}

pub fn new() -> GameLog {
    return GameLog {
        entries: vec![
            "Welcome Pilgrim! You seem lost...".to_string(),
        ],
    }
}
