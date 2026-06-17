use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use tui_2048::{
    app::{App, Direction, Screen},
    ui::grid::{render_game_over_popup, render_scores_popup},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    run(&mut app, &mut terminal)
}

pub fn run(app: &mut App, terminal: &mut DefaultTerminal) -> io::Result<()> {
    while !app.exit {
        terminal.draw(|frame| {
            draw(app, frame);
            if app.game_over {
                let _ = app.write_scores_to_file();
                render_game_over_popup(frame, app);
            }
            if app.showing_score {
                render_scores_popup(frame);
            }
        })?;
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
    }
}
