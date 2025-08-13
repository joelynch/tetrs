use std::io::{stdout, Stdout};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, BorderType, Borders},
    Terminal,
};

use crate::{positions::Position, tetramino_rules::Blocks, tetromino::Colour};

#[derive(Debug)]
pub struct UIState {
    pub game_over: bool,
    pub board: Vec<Vec<Option<Colour>>>,
    pub tetromino_blocks: Option<Blocks>,
    pub tetromino_colour: Option<Colour>,
    pub score: u32,
}

pub struct UI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl UI {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    pub fn start_ui(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.terminal
            .backend_mut()
            .execute(EnterAlternateScreen)?
            .execute(EnableMouseCapture)?;
        Ok(())
    }

    pub fn draw(&mut self, state: &UIState) -> Result<()> {
        self.terminal.draw(|frame| {
            let frame_rect = frame.size();
            let x_cells = state.board[0].len() as u16;
            let y_cells = state.board.len() as u16;
            let x_cell_size = (frame_rect.width / 3) / (x_cells + 2);
            let y_cell_size = frame_rect.height / (y_cells + 2);
            let border = Rect {
                x: frame_rect.width / 3,
                y: 0,
                width: x_cell_size * (x_cells + 2),
                height: y_cell_size * (y_cells + 2),
            };
            let game_area = Rect {
                x: border.x + x_cell_size,
                y: border.y + y_cell_size,
                width: border.width - (x_cell_size * 2),
                height: border.height - (y_cell_size * 2),
            };
            let title = if state.game_over {
                format!("GAME OVER :( ! Final Score: {}", state.score)
            } else {
                format!("TETRIS! Score: {}", state.score)
            };
            let outline = Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Double)
                .title(title)
                .title_alignment(Alignment::Center);

            frame.render_widget(outline, border);

            for (y, row) in state.board.iter().enumerate() {
                for (x, cell_colour) in row.iter().enumerate() {
                    let cell = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    let rect = Self::cell_rectangle(&cell, x_cells, y_cells, &game_area);
                    let mut block = Block::default().borders(Borders::NONE);

                    if let Some(blocks) = state.tetromino_blocks {
                        if blocks.contains(&cell) {
                            let style = Style::default()
                                .bg(Self::map_colour(&state.tetromino_colour.unwrap()));
                            block = block.style(style);
                        } else if let Some(colour) = cell_colour {
                            let style = Style::default().bg(Self::map_colour(colour));
                            block = block.style(style);
                        }
                    } else if let Some(colour) = cell_colour {
                        let style = Style::default().bg(Self::map_colour(colour));
                        block = block.style(style);
                    }
                    frame.render_widget(block, rect);
                }
            }
        })?;
        Ok(())
    }

    fn cell_rectangle(pos: &Position, max_x: u16, max_y: u16, game_area: &Rect) -> Rect {
        Rect {
            x: game_area.x + pos.x as u16 * game_area.width / max_x,
            y: game_area.y + pos.y as u16 * game_area.height / max_y,
            width: game_area.width / max_x,
            height: game_area.height / max_y,
        }
    }

    pub fn stop_ui(&mut self) -> Result<()> {
        self.terminal
            .backend_mut()
            .execute(LeaveAlternateScreen)?
            .execute(DisableMouseCapture)?;
        self.terminal.show_cursor()?;
        disable_raw_mode()?;
        Ok(())
    }

    fn map_colour(colour: &Colour) -> tui::style::Color {
        match colour {
            Colour::Blue => tui::style::Color::Blue,
            Colour::Cyan => tui::style::Color::Cyan,
            Colour::Green => tui::style::Color::Green,
            Colour::Magenta => tui::style::Color::Magenta,
            Colour::Red => tui::style::Color::Red,
            Colour::Yellow => tui::style::Color::Yellow,
        }
    }
}
