use std::ops::Deref;

#[test]

// struct
// tuple struct
// anonymous structs

fn structs() {
    let mut player_health = Health(100);
    let enemy_health = Health::new(100);

    player_health.lose_health(10);
    player_health.print_health();

    let ice_cream = FavoriteFood::IceCream {
        topping: "Nothing".to_owned(),
        scoops: 1,
        flavor: "Space Junkie".to_owned(),
    };
}

struct Health(pub u32);

impl Health {
    pub fn new(data: u32) -> Self {
        Self(data)
    }

    pub fn lose_health(&mut self, amount: u32) {
        self.0 -= amount;
    }

    pub fn print_health(&self) {
        println!("health: {:?}", **self);
    }
}

impl Deref for Health {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum FavoriteFood {
    Hamburger,
    HotDog,
    IceCream {
        topping: String,
        scoops: u32,
        flavor: String,
    },
}
