use std::path::PathBuf;

use ratatui::{
    Frame,
    layout::Constraint,
    style::Stylize,
    text::Line,
    widgets::{Block, Clear, Paragraph},
};

use crate::{
    SCORES_PATH,
    app::{App, read_scores_file},
};

pub fn render_game_over_popup(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let controls = Line::from(vec![
        " New Game ".into(),
        "<n>".blue().bold(),
        " High Scores ".into(),
        "<s> ".blue().bold(),
        " Quit ".into(),
        "<q> ".blue().bold(),
    ]);

    let popup_block = Block::bordered().title("Game Over").title_bottom(controls);
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    let paragraph =
        Paragraph::new(format!("You finished with a score of {}", app.score)).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}

pub fn render_scores_popup(frame: &mut Frame) {
    let area = frame.area();

    let controls = Line::from(vec![
        " New Game ".into(),
        "<n>".blue().bold(),
        " Hide Scores ".into(),
        "<s> ".blue().bold(),
        " Quit ".into(),
        "<q> ".blue().bold(),
    ]);

    let popup_block = Block::bordered()
        .title("High Scores")
        .title_bottom(controls);
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);

    let home = std::env::var("HOME").unwrap_or("~".to_string());

    let mut path = PathBuf::from(home);
    path.push(SCORES_PATH);

    let scores = read_scores_file(path);
    let paragraph = Paragraph::new(scores).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}

pub fn render_game_style_popup(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let controls = Line::from(vec![
        " Up ".into(),
        "<k>".blue().bold(),
        " Down ".into(),
        "<j> ".blue().bold(),
        " Select ".into(),
        "<Enter> ".blue().bold(),
    ]);

    let popup_block = Block::bordered()
        .title("Choose your game mode")
        .title_bottom(controls);
    let centered_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
    frame.render_widget(Clear, centered_area);
    let paragraph = Paragraph::new("Normal\nTimed 5min\nTimed 10min").block(popup_block);
    frame.render_widget(paragraph, centered_area);
}
