#![feature(core_intrinsics)]
use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use log::{info, LevelFilter};
use log4rs::{
    append::file::FileAppender,
    config::{Appender, Root},
    Config,
};
use ui::UI;

use anyhow::Result;
use clap::Parser;
use settings::Settings;

use crate::{
    game::Tetris,
    input::{input_loop, Action},
};

mod board;
mod game;
mod input;
mod positions;
mod settings;
mod spawner;
mod tetramino_rules;
mod tetromino;
mod ui;

fn main() -> Result<()> {
    let logfile = FileAppender::builder().build("log.txt")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    let settings = Settings::parse();
    info!("{:?}", settings);

    let mut ui = UI::new()?;
    let game = Tetris::new(settings.width, settings.height);
    ui.start_ui()?;
    ui.draw(&game.ui_state()).unwrap();

    let game = Arc::new(Mutex::new(game));
    let ui = Arc::new(Mutex::new(ui));

    thread::spawn({
        let game = game.clone();
        let ui = ui.clone();
        move || loop {
            sleep(Duration::from_millis((settings.interval * 1000.0) as u64));
            let mut game = game.lock().unwrap();
            game.update();
            let mut ui = ui.lock().unwrap();
            ui.draw(&game.ui_state()).unwrap();
        }
    });

    for action in input_loop() {
        let mut game = game.lock().unwrap();
        match action {
            Action::Quit => break,
            Action::RotateClockwise => game.rotate_clockwise(),
            Action::RotateAntiClockwise => game.rotate_anticlockwise(),
            Action::MoveLeft => game.move_tetromino_left(),
            Action::MoveRight => game.move_tetromino_right(),
            Action::Drop => game.drop_tetromino(),
            Action::MoveDown => game.move_tetromino_down(),
            Action::Switch => game.switch_tetromino(),
            Action::Restart => *game = Tetris::new(settings.width, settings.height),
        }
        let mut ui = ui.lock().unwrap();
        ui.draw(&game.ui_state())?;
    }

    ui.lock().unwrap().stop_ui()?;
    Ok(())
}
