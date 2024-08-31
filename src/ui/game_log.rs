pub struct GameLog {
    pub entries: Vec<String>,
}

pub fn new() -> GameLog {
    return GameLog {
        entries: vec![
            "You seem lost...".to_string(),
            "Welcome Pilgrim!".to_string(),
        ],
    }
}
