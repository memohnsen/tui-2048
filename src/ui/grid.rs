use ratatui::{
    Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Clear, Padding, Paragraph, Widget},
};

use crate::app::{App, read_scores_file};

#[derive(Debug, PartialEq)]
pub struct Grid {
    pub cells: [[u32; 4]; 4],
}

#[allow(non_camel_case_types)]
enum Colors {
    _0,
    _2,
    _4,
    _8,
    _16,
    _32,
    _64,
    _128,
    _256,
    _512,
    _1024,
    _2048,
    _4096,
    _8192,
}

pub fn get_bg_colors(num: String) -> Color {
    let matched_num = match num.as_str() {
        "2" => Colors::_2,
        "4" => Colors::_4,
        "8" => Colors::_8,
        "16" => Colors::_16,
        "32" => Colors::_32,
        "64" => Colors::_64,
        "128" => Colors::_128,
        "256" => Colors::_256,
        "512" => Colors::_512,
        "1024" => Colors::_1024,
        "2048" => Colors::_2048,
        "4096" => Colors::_4096,
        "8192" => Colors::_8192,
        _ => Colors::_0,
    };

    match matched_num {
        Colors::_0 => Color::White,
        Colors::_2 => Color::Rgb(238, 228, 218),
        Colors::_4 => Color::Rgb(237, 224, 200),
        Colors::_8 => Color::Rgb(242, 177, 121),
        Colors::_16 => Color::Rgb(245, 149, 99),
        Colors::_32 => Color::Rgb(246, 124, 95),
        Colors::_64 => Color::Rgb(246, 94, 59),
        Colors::_128 => Color::Rgb(237, 207, 114),
        Colors::_256 => Color::Rgb(237, 204, 97),
        Colors::_512 => Color::Rgb(237, 200, 80),
        Colors::_1024 => Color::Rgb(237, 197, 63),
        Colors::_2048 => Color::Rgb(237, 194, 46),
        Colors::_4096 => Color::Rgb(173, 216, 230),
        Colors::_8192 => Color::Rgb(147, 112, 219),
    }
}

impl Widget for &Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..4).map(|_| Constraint::Length(40));
        let row_constraints = (0..4).map(|_| Constraint::Length(10));
        let horizontal = Layout::horizontal(col_constraints).flex(ratatui::layout::Flex::Center);
        let vertical = Layout::vertical(row_constraints).flex(ratatui::layout::Flex::Center);

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

            let color = get_bg_colors(text.clone());

            Paragraph::new(text.bold().fg(Color::Black))
                .block(
                    Block::bordered()
                        .fg(color)
                        .bg(color)
                        .padding(Padding::vertical(3)),
                )
                .centered()
                .render(cell, buf);
        }
    }
}

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

// TODO:
// 1. allow this to be scrollable with jk to go through all results
// 2. add ability to change sorting between score and date
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

    let scores = read_scores_file();
    let paragraph = Paragraph::new(scores).block(popup_block);
    frame.render_widget(paragraph, centered_area);
}
