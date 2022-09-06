use rand::{
    distributions::{Distribution, Standard},
    rngs::StdRng,
    thread_rng, Rng, SeedableRng,
};

use crate::tetromino::{Colour, Orientation, Tetromino, TetrominoType};

#[derive(Debug)]
pub struct TetrominoSpawner {
    rng: StdRng,
}

impl Distribution<Colour> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Colour {
        match rng.gen_range(0..=5) {
            0 => Colour::Red,
            1 => Colour::Green,
            2 => Colour::Blue,
            3 => Colour::Yellow,
            4 => Colour::Cyan,
            5 => Colour::Magenta,
            _ => unreachable!(),
        }
    }
}

impl Distribution<Orientation> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Orientation {
        match rng.gen_range(0..=3) {
            0 => Orientation::Up,
            1 => Orientation::Right,
            2 => Orientation::Down,
            3 => Orientation::Left,
            _ => unreachable!(),
        }
    }
}

impl Distribution<TetrominoType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> TetrominoType {
        match rng.gen_range(0..=6) {
            0 => TetrominoType::I,
            1 => TetrominoType::J,
            2 => TetrominoType::L,
            3 => TetrominoType::O,
            4 => TetrominoType::S,
            5 => TetrominoType::T,
            6 => TetrominoType::Z,
            _ => unreachable!(),
        }
    }
}

impl TetrominoSpawner {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_rng(thread_rng()).unwrap(),
        }
    }

    pub fn spawn(&mut self) -> Tetromino {
        let colour = self.rng.gen();
        let type_ = self.rng.gen();
        let orientation = self.rng.gen();
        Tetromino::new(5, colour, type_, orientation)
    }
}
