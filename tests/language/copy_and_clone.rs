use std::ops::Add;

#[test]
fn copy_and_clone() {
    let player_location = Vector2 { x: 10.0, y: 15.0 };
    let player_velocity = Vector2 { x: 1.0, y: 2.0 };
    let _new_location = player_location + player_velocity;
    let _new_location = player_location + player_velocity;
}

#[derive(Debug, Clone, Copy)]
struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
