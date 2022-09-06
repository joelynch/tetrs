use strum::EnumIter;

use crate::positions::Position;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, EnumIter)]
pub enum Colour {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, EnumIter)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, EnumIter)]
pub enum Orientation {
    Up,
    Right,
    Down,
    Left,
}

impl Orientation {
    pub fn rotate_clockwise(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }

    pub fn rotate_anticlockwise(&self) -> Orientation {
        match self {
            Orientation::Up => Orientation::Left,
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Tetromino {
    pub orientation: Orientation,
    pub colour: Colour,
    pub type_: TetrominoType,
    pub box_corner: Position,
}

impl Tetromino {
    pub fn new(x: i32, colour: Colour, orientation: Orientation, type_: TetrominoType) -> Self {
        Self {
            box_corner: Position { x, y: 0 },
            orientation,
            colour,
            type_,
        }
    }
}
