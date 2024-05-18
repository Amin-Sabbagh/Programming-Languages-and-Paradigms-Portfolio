// player.rs
pub struct Player {
    pub health: i32,
}

impl Player {
    pub fn new(health: i32) -> Self {
        Player { health }
    }
}