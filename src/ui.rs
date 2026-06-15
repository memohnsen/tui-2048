use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::app::{App, Screen};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" 2048 ".bold());
        let instructions = match self.current_screen {
            Screen::Playing => Line::from(vec![
                " Move ".into(),
                "<hjkl>".blue().bold(),
                " New Game ".into(),
                "<n>".blue().bold(),
                " High Scores ".into(),
                "<s> ".blue().bold(),
                " Quit ".into(),
                "<q> ".blue().bold(),
            ]),
            Screen::GameOver => Line::from(vec![
                " New Game ".into(),
                "<n>".blue().bold(),
                " High Scores ".into(),
                "<s> ".blue().bold(),
                " Quit ".into(),
                "<q> ".blue().bold(),
            ]),
        };
        let block = Block::bordered()
            .title(title.centered())
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
            self.high_score.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .render(score_area, buf);

        self.grid.render(grid_area, buf);
    }
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    pub cells: [[u32; 4]; 4],
}

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..4).map(|_| Constraint::Length(45));
        let row_constraints = (0..4).map(|_| Constraint::Length(20));
        let horizontal = Layout::horizontal(col_constraints);
        let vertical = Layout::vertical(row_constraints);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            let row = i / 4;
            let col = i % 4;
            let value = self.cells[row][col];

            let text = if value == 0 {
                String::new()
            } else {
                value.to_string()
            };

            Paragraph::new(text.bold())
                .block(Block::bordered())
                .centered()
                .render(cell, buf);
        }
    }
}
