use std::collections::HashMap;

use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};
use strum::IntoEnumIterator;

use crate::{
    positions::Position,
    tetromino::{Orientation, Tetromino, TetrominoType},
};

pub type Blocks = [Position; 4];
type BlockOffsets = [Position; 4];
type PositionLookup = HashMap<(Orientation, Orientation), Vec<Position>>;
type OffsetsMap = HashMap<(TetrominoType, Orientation), BlockOffsets>;

#[derive(Debug)]
pub struct TetrominoRules {
    jltsz_kicks_lookup: PositionLookup,
    i_kicks_lookup: PositionLookup,
    offsets_lookup: OffsetsMap,
    empty: Vec<Position>,
    rng: StdRng,
}

impl TetrominoRules {
    pub fn new() -> Self {
        Self {
            jltsz_kicks_lookup: Self::build_jltsz_kicks_lookup(),
            i_kicks_lookup: Self::build_i_kicks_lookup(),
            offsets_lookup: Self::build_offsets_lookup(),
            empty: vec![],
            rng: StdRng::from_rng(thread_rng()).unwrap(),
        }
    }

    pub fn rotate(&self, tetromino: &Tetromino, new_orientation: Orientation) -> Vec<Tetromino> {
        let base_rotation = Tetromino {
            orientation: new_orientation,
            ..tetromino.clone()
        };
        let mut rotations = vec![base_rotation.clone()];
        for kick in self.kicks(&base_rotation, new_orientation) {
            let kick_rotation = Tetromino {
                box_corner: &tetromino.box_corner + kick,
                ..base_rotation.clone()
            };
            rotations.push(kick_rotation);
        }
        rotations
    }

    pub fn move_down(&self, tetromino: &Tetromino) -> Tetromino {
        Tetromino {
            box_corner: tetromino.box_corner + Position { x: 0, y: 1 },
            ..tetromino.clone()
        }
    }

    pub fn move_right(&self, tetromino: &Tetromino) -> Tetromino {
        Tetromino {
            box_corner: tetromino.box_corner + Position { x: 1, y: 0 },
            ..tetromino.clone()
        }
    }

    pub fn move_left(&self, tetromino: &Tetromino) -> Tetromino {
        Tetromino {
            box_corner: tetromino.box_corner + Position { x: -1, y: 0 },
            ..tetromino.clone()
        }
    }

    pub fn switch(&mut self, tetromino: &Tetromino) -> Tetromino {
        Tetromino {
            type_: self.rng.gen(),
            colour: self.rng.gen(),
            ..tetromino.clone()
        }
    }

    pub fn blocks(&self, tetromino: &Tetromino) -> Blocks {
        self.offsets(tetromino)
            .map(|offset| tetromino.box_corner + offset)
    }

    fn offsets(&self, tetromino: &Tetromino) -> &BlockOffsets {
        self.offsets_lookup
            .get(&(tetromino.type_, tetromino.orientation))
            .unwrap()
    }

    fn box_size(type_: TetrominoType) -> u32 {
        match type_ {
            TetrominoType::I => 4,
            TetrominoType::O => 2,
            _ => 3,
        }
    }

    fn kicks(&self, tetromino: &Tetromino, new_orientation: Orientation) -> &Vec<Position> {
        match tetromino.type_ {
            TetrominoType::I => self
                .i_kicks_lookup
                .get(&(tetromino.orientation, new_orientation))
                .unwrap_or(&self.empty),
            TetrominoType::O => &self.empty,
            _ => self
                .jltsz_kicks_lookup
                .get(&(tetromino.orientation, new_orientation))
                .unwrap_or(&self.empty),
        }
    }

    fn build_jltsz_kicks_lookup() -> PositionLookup {
        let jltsz_0_1_kicks = vec![
            Position { x: -1, y: 0 },
            Position { x: -1, y: 1 },
            Position { x: 0, y: -2 },
            Position { x: -1, y: -2 },
        ];
        let jltsz_1_0_kicks = vec![
            Position { x: 1, y: 0 },
            Position { x: 1, y: -1 },
            Position { x: 0, y: -2 },
            Position { x: 1, y: 2 },
        ];
        let jltsz_1_2_kicks = vec![
            Position { x: 1, y: 0 },
            Position { x: 1, y: -1 },
            Position { x: 0, y: 2 },
            Position { x: 1, y: 2 },
        ];
        let jltsz_2_1_kicks = vec![
            Position { x: -1, y: 0 },
            Position { x: -1, y: 1 },
            Position { x: 0, y: -2 },
            Position { x: -1, y: -2 },
        ];
        let jltsz_2_3_kicks = vec![
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 0, y: -2 },
            Position { x: 1, y: -2 },
        ];
        let jltsz_3_2_kicks = vec![
            Position { x: -1, y: 0 },
            Position { x: -1, y: -1 },
            Position { x: 0, y: -2 },
            Position { x: -1, y: 2 },
        ];
        let jltsz_3_0_kicks = vec![
            Position { x: -1, y: 0 },
            Position { x: -1, y: -1 },
            Position { x: 0, y: 2 },
            Position { x: -1, y: 2 },
        ];
        let jltsz_0_3_kicks = vec![
            Position { x: 1, y: 0 },
            Position { x: 1, y: 1 },
            Position { x: 0, y: -2 },
            Position { x: 1, y: -2 },
        ];
        HashMap::from([
            ((Orientation::Up, Orientation::Right), jltsz_0_1_kicks),
            ((Orientation::Right, Orientation::Up), jltsz_1_0_kicks),
            ((Orientation::Right, Orientation::Down), jltsz_1_2_kicks),
            ((Orientation::Down, Orientation::Right), jltsz_2_1_kicks),
            ((Orientation::Down, Orientation::Left), jltsz_2_3_kicks),
            ((Orientation::Left, Orientation::Down), jltsz_3_2_kicks),
            ((Orientation::Left, Orientation::Up), jltsz_3_0_kicks),
            ((Orientation::Up, Orientation::Left), jltsz_0_3_kicks),
        ])
    }

    fn build_i_kicks_lookup() -> PositionLookup {
        let i_0_1_kicks = vec![
            Position { x: -2, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: -2, y: -1 },
            Position { x: 1, y: 2 },
        ];
        let i_1_0_kicks = vec![
            Position { x: 2, y: 0 },
            Position { x: -1, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: -1, y: -2 },
        ];
        let i_1_2_kicks = vec![
            Position { x: -1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: -1, y: 2 },
            Position { x: 2, y: -1 },
        ];
        let i_2_1_kicks = vec![
            Position { x: 1, y: 0 },
            Position { x: -2, y: 0 },
            Position { x: 1, y: -2 },
            Position { x: -2, y: 1 },
        ];
        let i_2_3_kicks = vec![
            Position { x: 2, y: 0 },
            Position { x: -1, y: 0 },
            Position { x: 2, y: 1 },
            Position { x: -1, y: -2 },
        ];
        let i_3_2_kicks = vec![
            Position { x: -2, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: -2, y: -1 },
            Position { x: 1, y: 2 },
        ];
        let i_3_0_kicks = vec![
            Position { x: 1, y: 0 },
            Position { x: -2, y: 0 },
            Position { x: 1, y: -2 },
            Position { x: -2, y: 1 },
        ];
        let i_0_3_kicks = vec![
            Position { x: -1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: -1, y: 2 },
            Position { x: 2, y: -1 },
        ];
        HashMap::from([
            ((Orientation::Up, Orientation::Right), i_0_1_kicks),
            ((Orientation::Right, Orientation::Up), i_1_0_kicks),
            ((Orientation::Right, Orientation::Down), i_1_2_kicks),
            ((Orientation::Down, Orientation::Right), i_2_1_kicks),
            ((Orientation::Down, Orientation::Left), i_2_3_kicks),
            ((Orientation::Left, Orientation::Down), i_3_2_kicks),
            ((Orientation::Left, Orientation::Up), i_3_0_kicks),
            ((Orientation::Up, Orientation::Left), i_0_3_kicks),
        ])
    }

    fn rotate_offset_90(pos: Position, box_size: u32) -> Position {
        Position {
            x: box_size as i32 - pos.y - 1,
            y: pos.x,
        }
    }

    fn rotate_offset(pos: Position, orientation: Orientation, box_size: u32) -> Position {
        let rotate = move |arg| Self::rotate_offset_90(arg, box_size);
        match orientation {
            Orientation::Up => pos,
            Orientation::Right => rotate(pos),
            Orientation::Down => rotate(rotate(pos)),
            Orientation::Left => rotate(rotate(rotate(pos))),
        }
    }

    fn rotate_offsets(
        offsets: &BlockOffsets,
        orientation: Orientation,
        box_size: u32,
    ) -> BlockOffsets {
        offsets
            .iter()
            .map(|pos| Self::rotate_offset(*pos, orientation, box_size))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn build_offsets_lookup() -> HashMap<(TetrominoType, Orientation), BlockOffsets> {
        let orientations = HashMap::from([
            (
                TetrominoType::I,
                [
                    Position { x: 0, y: 1 },
                    Position { x: 1, y: 1 },
                    Position { x: 2, y: 1 },
                    Position { x: 3, y: 1 },
                ],
            ),
            (
                TetrominoType::J,
                [
                    Position { x: 0, y: 0 },
                    Position { x: 0, y: 1 },
                    Position { x: 1, y: 1 },
                    Position { x: 2, y: 1 },
                ],
            ),
            (
                TetrominoType::L,
                [
                    Position { x: 0, y: 1 },
                    Position { x: 1, y: 1 },
                    Position { x: 2, y: 1 },
                    Position { x: 2, y: 0 },
                ],
            ),
            (
                TetrominoType::O,
                [
                    Position { x: 0, y: 0 },
                    Position { x: 0, y: 1 },
                    Position { x: 1, y: 0 },
                    Position { x: 1, y: 1 },
                ],
            ),
            (
                TetrominoType::S,
                [
                    Position { x: 0, y: 1 },
                    Position { x: 1, y: 1 },
                    Position { x: 1, y: 0 },
                    Position { x: 2, y: 0 },
                ],
            ),
            (
                TetrominoType::T,
                [
                    Position { x: 0, y: 1 },
                    Position { x: 1, y: 1 },
                    Position { x: 1, y: 0 },
                    Position { x: 2, y: 1 },
                ],
            ),
            (
                TetrominoType::Z,
                [
                    Position { x: 0, y: 0 },
                    Position { x: 1, y: 0 },
                    Position { x: 1, y: 1 },
                    Position { x: 2, y: 1 },
                ],
            ),
        ]);

        TetrominoType::iter()
            .flat_map(|type_| {
                let offsets = orientations.get(&type_).unwrap();
                Orientation::iter().map(move |or| {
                    (
                        (type_, or),
                        Self::rotate_offsets(offsets, or, Self::box_size(type_)),
                    )
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::TetrominoRules;
    use crate::positions::Position;

    #[test]
    fn test_rotate_90() {
        assert_eq!(
            TetrominoRules::rotate_offset_90(Position { x: 0, y: 0 }, 4),
            Position { x: 0, y: 3 }
        );
        assert_eq!(
            TetrominoRules::rotate_offset_90(Position { x: 1, y: 1 }, 4),
            Position { x: 1, y: 2 }
        );
        assert_eq!(
            TetrominoRules::rotate_offset_90(Position { x: 1, y: 1 }, 3),
            Position { x: 1, y: 1 }
        );
        assert_eq!(
            TetrominoRules::rotate_offset_90(Position { x: 1, y: 1 }, 2),
            Position { x: 1, y: 0 }
        );
    }
}
