use std::{io, path::PathBuf};

use crate::{
    SCORES_PATH,
    app::{App, Direction, Screen, write_scores_to_file},
    ui::popups::{render_game_over_popup, render_game_style_popup, render_scores_popup},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| {
            draw(app, frame);

            if !app.chosen_game_style {
                render_game_style_popup(frame, app);
            }

            if app.game_over {
                let home = std::env::var("HOME").unwrap_or("~".to_string());

                let mut path = PathBuf::from(home);
                path.push(SCORES_PATH);

                let _ = write_scores_to_file(app, path);
                render_game_over_popup(frame, app);
            }
            if app.showing_score {
                render_scores_popup(frame);
            }
        })?;
        handle_events(app)?;
    }
    ratatui::restore();
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
            KeyCode::Char('l') => app.move_nums(Direction::Right),
            KeyCode::Char('h') => app.move_nums(Direction::Left),
            KeyCode::Char('j') => app.move_nums(Direction::Down),
            KeyCode::Char('k') => app.move_nums(Direction::Up),
            KeyCode::Right => app.move_nums(Direction::Right),
            KeyCode::Left => app.move_nums(Direction::Left),
            KeyCode::Down => app.move_nums(Direction::Down),
            KeyCode::Up => app.move_nums(Direction::Up),
            KeyCode::Char('s') => app.toggle_scores(),
            KeyCode::Char('n') => app.new_game(),
            _ => {}
        },
        Screen::GameOver => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.toggle_scores(),
            KeyCode::Char('n') => app.new_game(),
            _ => {}
        },
        Screen::Scores => match key_event.code {
            KeyCode::Char('q') => app.exit(),
            KeyCode::Char('s') => app.toggle_scores(),
            _ => {}
        },
        Screen::GameStyle => match key_event.code {
            KeyCode::Char('n') => app.new_game(),
            KeyCode::Char('j') => {
                todo!()
            }
            KeyCode::Char('k') => {
                todo!()
            }
            _ => {}
        },
    }
}
