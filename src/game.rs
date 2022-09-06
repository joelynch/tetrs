use log::info;

use crate::{
    board::Board, spawner::TetrominoSpawner, tetramino_rules::TetrominoRules, tetromino::Tetromino,
    ui::UIState,
};

#[derive(Debug)]
pub struct Tetris {
    board: Board,
    current_tet: Option<Tetromino>,
    spawner: TetrominoSpawner,
    rules: TetrominoRules,
    pub score: u32,
    pub game_over: bool,
}

impl Tetris {
    pub fn new(rows: u32, cols: u32) -> Self {
        Self {
            board: Board::new(rows, cols),
            current_tet: None::<Tetromino>,
            spawner: TetrominoSpawner::new(),
            rules: TetrominoRules::new(),
            score: 0,
            game_over: false,
        }
    }

    pub fn update(&mut self) {
        info!("updating");
        if self.game_over {
            return;
        }

        if self.current_tet.is_some() {
            self.move_tetromino_down();
        } else {
            let new = self.spawner.spawn();
            info!("new tetromino: {:?}", new);
            if self.valid(&new) {
                self.current_tet = Some(new);
            } else {
                info!("game over");
                self.game_over = true;
            }
        }
    }

    pub fn move_tetromino_down(&mut self) {
        info!("moving tetromino down");
        if let Some(ref tet) = self.current_tet && !self.game_over  {
            let new = self.rules.move_down(tet);

            if !self.valid(&new) {
                self.score += Self::score_for_lines(
                    self.board.add_blocks(&self.rules.blocks(tet), tet.colour)
                );
                self.current_tet = None;
            } else {
                self.current_tet = Some(new);
            }
        }
    }

    pub fn rotate_clockwise(&mut self) {
        info!("rotating tetromino clockwise");
        if let Some(ref tet) = self.current_tet && !self.game_over {
            let new_orientation = tet.orientation.rotate_clockwise();

            let new = self.rules.rotate(tet, new_orientation).into_iter().find(|tet| {
                self.board.valid_position(&self.rules.blocks(tet))
            });

            if new.is_some() {
                self.current_tet = new;
            }
        }
    }

    pub fn rotate_anticlockwise(&mut self) {
        info!("rotating tetromino anti clockwise");
        if let Some(ref tet) = self.current_tet && !self.game_over {
            let new_orientation = tet.orientation.rotate_anticlockwise();

            let new = self.rules.rotate(tet, new_orientation).into_iter().find(|tet| self.valid(tet));

            if new.is_some() {
                self.current_tet = new;
            }
        }
    }

    pub fn move_tetromino_left(&mut self) {
        info!("moving tetromino left");
        if let Some(ref tet) = self.current_tet && !self.game_over {
            let new = self.rules.move_left(tet);

            if self.valid(&new) {
                self.current_tet = Some(new);
            }
        }
    }

    pub fn move_tetromino_right(&mut self) {
        info!("moving tetromino right");
        if let Some(ref tet) = self.current_tet && !self.game_over {
            let new = self.rules.move_right(tet);

            if self.valid(&new) {
                self.current_tet = Some(new);
            }
        }
    }

    pub fn drop_tetromino(&mut self) {
        info!("dropping tetromino");
        while self.current_tet.is_some() && !self.game_over {
            self.move_tetromino_down();
        }
    }

    pub fn switch_tetromino(&mut self) {
        info!("switching tetromino");
        if let Some(ref tet) = self.current_tet && !self.game_over {
            let new = self.rules.switch(tet);

            if self.valid(&new) {
                self.current_tet = Some(new);
            }
        }
    }

    pub fn ui_state(&self) -> UIState {
        let state = UIState {
            board: self.board.draw(),
            tetromino_blocks: self.current_tet.as_ref().map(|t| self.rules.blocks(t)),
            game_over: self.game_over,
            tetromino_colour: self.current_tet.as_ref().map(|t| t.colour),
            score: self.score,
        };
        info!("ui state: {:?}", state);
        state
    }

    fn valid(&self, tetromino: &Tetromino) -> bool {
        self.board.valid_position(&self.rules.blocks(tetromino))
    }

    fn score_for_lines(lines: u32) -> u32 {
        match lines {
            0 => 0,
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => panic!("invalid number of lines"),
        }
    }
}
