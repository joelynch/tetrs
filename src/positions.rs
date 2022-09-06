use std::ops::Add;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Add for &Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Position {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
