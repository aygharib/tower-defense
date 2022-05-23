struct Enemy {
    health: i32
}

impl Enemy {
    fn new(health: i32) -> Enemy {
        Enemy {
            health
        }
    }

    fn getHealth(&self) -> i32 {
        self.health
    }
}