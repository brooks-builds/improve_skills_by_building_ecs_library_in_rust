#[test]
fn default() {
    let player = Player::new();
    dbg!(player);
}

#[derive(Debug, Default)]
struct Player {
    health: Health,
    damage: u32,
}

impl Player {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug)]
struct Health(u32);

impl Default for Health {
    fn default() -> Self {
        Self(100)
    }
}
