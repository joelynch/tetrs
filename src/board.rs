use crate::{positions::Position, tetramino_rules::Blocks, tetromino::Colour};

#[derive(Debug, Clone)]
pub struct Cell {
    pub colour: Option<Colour>,
}

impl Cell {
    pub fn new() -> Self {
        Self { colour: None }
    }

    pub fn set_colour(&mut self, colour: Colour) {
        self.colour = Some(colour);
    }

    pub fn filled(&self) -> bool {
        self.colour.is_some()
    }
}

#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            cells: Self::new_grid(width, height),
        }
    }

    pub fn valid_position(&self, blocks: &Blocks) -> bool {
        blocks.iter().all(|block| {
            self.get_cell(block)
                .map(|cell| !cell.filled())
                .unwrap_or(false)
        })
    }

    pub fn add_blocks(&mut self, blocks: &Blocks, colour: Colour) -> u32 {
        blocks.iter().for_each(|block| {
            self.get_cell_mut(block).unwrap().set_colour(colour);
        });
        self.remove_full_rows()
    }

    pub fn draw(&self) -> Vec<Vec<Option<Colour>>> {
        self.cells
            .iter()
            .map(|row| row.iter().map(|cell| cell.colour).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn remove_full_rows(&mut self) -> u32 {
        let mut removed = 0;
        let mut new = Vec::with_capacity(self.height as usize);
        for row in self.cells.iter().rev() {
            if row.iter().all(|cell| cell.filled()) {
                removed += 1;
            } else {
                new.push(row.clone());
            }
        }
        new.resize_with(self.height as usize, || Self::new_row(self.width));
        new.reverse();
        self.cells = new;
        removed
    }

    fn get_cell(&self, pos: &Position) -> Option<&Cell> {
        self.cells
            .get(pos.y as usize)
            .and_then(|r| r.get(pos.x as usize))
    }

    fn get_cell_mut(&mut self, pos: &Position) -> Option<&mut Cell> {
        self.cells
            .get_mut(pos.y as usize)
            .and_then(|r| r.get_mut(pos.x as usize))
    }

    fn new_row(width: u32) -> Vec<Cell> {
        vec![Cell::new(); width as usize]
    }

    fn new_grid(width: u32, height: u32) -> Vec<Vec<Cell>> {
        let mut new = Vec::with_capacity(height as usize);
        new.resize_with(height as usize, || Self::new_row(width));
        new
    }
}
