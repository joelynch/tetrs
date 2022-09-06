use crossterm::event::{self, Event, KeyCode};

pub enum Action {
    RotateClockwise,
    RotateAntiClockwise,
    MoveLeft,
    MoveDown,
    MoveRight,
    Drop,
    Switch,
    Restart,
    Quit,
}

fn map_event(event: Event) -> Option<Action> {
    match event {
        Event::Key(key) => match key.code {
            KeyCode::Char('q') => Some(Action::Quit),
            KeyCode::Char('a') => Some(Action::RotateAntiClockwise),
            KeyCode::Char('d') => Some(Action::RotateClockwise),
            KeyCode::Char('j') => Some(Action::MoveLeft),
            KeyCode::Char('k') => Some(Action::MoveDown),
            KeyCode::Char('l') => Some(Action::MoveRight),
            KeyCode::Char('s') => Some(Action::Switch),
            KeyCode::Char('r') => Some(Action::Restart),
            KeyCode::Char(' ') => Some(Action::Drop),
            KeyCode::Right => Some(Action::MoveRight),
            KeyCode::Left => Some(Action::MoveLeft),
            KeyCode::Up => Some(Action::RotateAntiClockwise),
            KeyCode::Down => Some(Action::MoveDown),
            _ => None,
        },
        _ => None,
    }
}

pub fn input_loop() -> impl Iterator<Item = Action> {
    std::iter::from_fn(|| loop {
        if let Some(action) = map_event(event::read().unwrap()) {
            return Some(action);
        }
    })
}
