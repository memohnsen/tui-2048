use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::{App, Screen, read_scores_file};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" 2048 ".bold());
        let instructions = match self.current_screen {
            Screen::Playing => Line::from(vec![
                " Move ".into(),
                "<hjkl or Arrow Keys>".blue().bold(),
                " New Game ".into(),
                "<n>".blue().bold(),
                " High Scores ".into(),
                "<s> ".blue().bold(),
                " Quit ".into(),
                "<q> ".blue().bold(),
            ]),
            Screen::GameOver => Line::from(vec![]),
            Screen::Scores => Line::from(vec![]),
        };
        let block = Block::bordered()
            .title(title.centered().bold())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        let [score_area, grid_area] =
            Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).areas(inner);

        let counter_text = Text::from(vec![Line::from(vec![
            "Score: ".into(),
            self.score.to_string().yellow(),
            " | High Score: ".into(),
            get_highest_score().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .render(score_area, buf);

        self.grid.render(grid_area, buf);
    }
}

pub fn get_highest_score() -> String {
    let scores = read_scores_file();
    let score_lines: Vec<&str> = scores.lines().skip(1).collect();

    let mut high_score: Vec<&str> = Vec::new();

    for line in score_lines {
        let score_from_entry: Vec<&str> = line.split(" ").collect();
        high_score.push(score_from_entry[2]);
    }

    match high_score.iter().max() {
        Some(&max) => max.to_string(),
        _ => "0".to_string(),
    }
}
