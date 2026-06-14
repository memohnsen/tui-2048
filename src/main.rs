use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tui_2048::app::{App, Screen};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    run(&mut app, &mut terminal)
}

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| draw(app, frame))?;
        handle_events(app)?;
    }
    Ok(())
}

pub fn draw(app: &App, frame: &mut Frame) {
    frame.render_widget(app, frame.area());
}

pub fn handle_events(app: &mut App) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            handle_key_event(app, key_event)
        }
        _ => {}
    };
    Ok(())
}

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match app.current_screen {
        Screen::Playing => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('l') => app.move_right(),
            KeyCode::Char('h') => app.move_left(),
            KeyCode::Char('j') => app.move_down(),
            KeyCode::Char('k') => app.move_up(),
            KeyCode::Char('s') => app.show_scores(),
            KeyCode::Char('n') => app.new_game(),
            // NOTE: dev only remove before release
            KeyCode::Char('f') => app.full_tiles(),
            _ => {}
        },
        Screen::GameOver => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.show_scores(),
            KeyCode::Char('n') => app.new_game(),
            _ => {}
        },
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use ratatui::style::Style;

    // #[test]
    // fn handle_key_event() {
    //     let mut app = App::default();
    //     app.handle_key_event(KeyCode::Right.into());
    //     assert_eq!(app.counter, 1);

    //     app.handle_key_event(KeyCode::Left.into());
    //     assert_eq!(app.counter, 0);

    //     let mut app = App::default();
    //     app.handle_key_event(KeyCode::Char('q').into());
    //     assert!(app.exit);
    // }
}
