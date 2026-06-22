#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use ratatui::{Terminal, backend::TestBackend};

    use crate::{
        app::{App, GameStyle, Screen},
        ui::grid::Grid,
    };

    fn build_app() -> App {
        App {
            highest_num: 0,
            score: 0,
            game_over: false,
            showing_score: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]],
            },
            current_screen: Screen::Playing,
            game_style: GameStyle::Normal,
            chosen_game_style: true,
        }
    }

    #[test]
    fn test_render_app() {
        // set_var is unsafe in Rust 1.80+ because modifying environment variables
        // while other threads might be reading them causes data races.
        // It is considered acceptable here for a single test environment if it's the only one
        // needing HOME, or if run with `cargo test -- --test-threads=1`.
        unsafe {
            std::env::set_var("HOME", env!("CARGO_MANIFEST_DIR"));
        }
        let app = build_app();
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
