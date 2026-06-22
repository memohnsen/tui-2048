use std::io;

use tui_2048::{app::App, cli::run::run_command, terminal::run};

fn main() -> io::Result<()> {
    let argv: Vec<String> = std::env::args().skip(1).collect();
    if !argv.is_empty() {
        run_command(argv);
        return Ok(());
    }

    let mut terminal = ratatui::init();
    let mut app = App::default();
    run(&mut app, &mut terminal)
}
