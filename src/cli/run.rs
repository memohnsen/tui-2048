use std::{io, path::PathBuf};

use crate::{
    SCORES_PATH,
    app::{App, read_scores_file},
    terminal::run,
};

pub enum Commands {
    Scores(String),
    New(String),
    Help,
    Other,
}

impl Commands {
    pub fn execute(&self) -> io::Result<()> {
        match self {
            Commands::Scores(subcommand) => {
                let home = std::env::var("HOME").unwrap_or("~".to_string());
                let mut path = PathBuf::from(home);
                path.push(SCORES_PATH);

                let scores = read_scores_file(path);
                let header = scores
                    .lines()
                    .next()
                    .unwrap_or("Date Score Highest Num Game Style");
                let scores_lines: Vec<&str> = scores.lines().skip(1).collect();

                println!("{header}");
                for line in scores_lines {
                    if line.split(" ").nth(4).unwrap() == subcommand {
                        println!("{line}");
                    }
                }
                Ok(())
            }
            Commands::New(subcommand) => match subcommand.as_str() {
                "timed-5" => {
                    let mut terminal = ratatui::init();
                    let mut app = App::default();
                    run(&mut app, &mut terminal)?;
                    Ok(())
                }
                "timed-10" => {
                    let mut terminal = ratatui::init();
                    let mut app = App::default();
                    run(&mut app, &mut terminal)?;
                    Ok(())
                }
                &_ => {
                    let mut terminal = ratatui::init();
                    let mut app = App::default();
                    run(&mut app, &mut terminal)?;
                    Ok(())
                }
            },
            Commands::Help => {
                help();
                Ok(())
            }
            Commands::Other => {
                println!(
                    "Not a valid command. Type tui-2048 help to see a list of all available commands"
                );
                Ok(())
            }
        }
    }
}

pub fn run_command(command: Vec<String>) {
    let subcommand = if command.len() == 2 {
        command[1].clone().to_string()
    } else {
        "".to_string()
    };

    match command[0].as_str() {
        "scores" => Commands::Scores(subcommand).execute(),
        "new" => Commands::New(subcommand).execute(),
        "help" => Commands::Help.execute(),
        &_ => Commands::Other.execute(),
    };
}

pub fn help() {
    println!("tui-2048 usage");
    println!("--------------");
    println!("COMMANDS");
    println!("scores                lists all scores for the default game style");
    println!("scores timed-5        lists all scores for the 5min timed game style");
    println!("scores timed-10       lists all scores for the 10min timed game style");
    println!();
    println!("new timed-5           starts a new timed 5min game");
    println!("new timed-10          starts a new timed 10min game");
}
