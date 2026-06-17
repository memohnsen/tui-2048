use std::path::PathBuf;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::{
    SCORES_PATH,
    app::{App, Screen, read_scores_file},
};

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

        let home = std::env::var("HOME").unwrap_or("~".to_string());

        let mut path = PathBuf::from(home);
        path.push(SCORES_PATH);

        let counter_text = Text::from(vec![Line::from(vec![
            "Score: ".into(),
            self.score.to_string().yellow(),
            " | High Score: ".into(),
            get_highest_score(path).yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .render(score_area, buf);

        self.grid.render(grid_area, buf);
    }
}

pub fn get_highest_score(path: PathBuf) -> String {
    let scores = read_scores_file(path);
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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{app::write_scores_to_file, ui::grid::Grid};

    use super::*;

    fn build_app() -> App {
        App {
            highest_num: 0,
            score: 200,
            game_over: false,
            showing_score: false,
            high_score: 200,
            exit: false,
            grid: Grid {
                cells: [[0, 0, 0, 0], [0, 0, 2, 2], [0, 2, 0, 0], [0, 0, 0, 0]],
            },
            current_screen: Screen::Playing,
        }
    }

    #[test]
    fn test_get_highest_score() {
        let path = PathBuf::from("./scores_ui_test.txt");
        let _ = fs::remove_file(&path);

        let score = get_highest_score(path.clone());
        assert_eq!(score, "0".to_string());

        let mut app = build_app();

        write_scores_to_file(&mut app, path.clone()).unwrap();

        let expected = format!(
            "Date Score Highest Num\n{} 200 0\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M"),
        );
        let contents = read_scores_file(path);

        assert_eq!(contents, expected);
        fs::remove_file("./scores_ui_test.txt").unwrap();
    }
}
