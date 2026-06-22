use core::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::{fs::OpenOptions, io::Result};

use rand::{
    distr::{Bernoulli, Distribution},
    random_range, rng,
};

use crate::ui::grid::Grid;

pub struct App {
    pub highest_num: u8,
    pub score: u32,
    pub showing_score: bool,
    pub game_over: bool,
    pub high_score: u32,
    pub exit: bool,
    pub grid: Grid,
    pub current_screen: Screen,
    pub game_style: GameStyle,
    pub chosen_game_style: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStyle {
    Normal,
    Timed5,
    Timed10,
}

impl GameStyle {
    pub fn as_str(self) -> &'static str {
        match self {
            GameStyle::Normal => "normal",
            GameStyle::Timed5 => "timed5",
            GameStyle::Timed10 => "timed10",
        }
    }
}

impl fmt::Display for GameStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for GameStyle {
    type Err = ();
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "normal" => Ok(GameStyle::Normal),
            "timed5" => Ok(GameStyle::Timed5),
            "timed10" => Ok(GameStyle::Timed10),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Screen {
    Playing,
    GameOver,
    Scores,
    GameStyle,
}

#[derive(PartialEq, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Default for App {
    fn default() -> Self {
        Self {
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
            chosen_game_style: false,
        }
    }
}

impl App {
    pub fn new_game(&mut self) {
        self.grid.cells = [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]];
        self.score = 0;
        self.highest_num = 0;
        self.game_over = false;
        self.current_screen = Screen::Playing;
        self.game_style = GameStyle::Normal;
    }

    pub fn new_game_timed5(&mut self) {
        self.grid.cells = [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]];
        self.score = 0;
        self.highest_num = 0;
        self.game_over = false;
        self.current_screen = Screen::Playing;
        self.game_style = GameStyle::Timed5;
    }

    pub fn new_game_timed10(&mut self) {
        self.grid.cells = [[0, 0, 0, 0], [0, 0, 2, 0], [0, 2, 0, 0], [0, 0, 0, 0]];
        self.score = 0;
        self.highest_num = 0;
        self.game_over = false;
        self.current_screen = Screen::Playing;
        self.game_style = GameStyle::Timed10;
    }

    pub fn move_nums(&mut self, direction: Direction) {
        let grid: Vec<u32> = self.grid.cells.iter().flatten().cloned().collect();

        match direction {
            Direction::Up => {
                let new = merge_row_vertical(self, Direction::Up);

                if self.grid == new && !grid.contains(&0) {
                    self.game_over = true;
                    self.current_screen = Screen::GameOver;
                } else if self.grid == new {
                    self.grid = merge_row_vertical(self, Direction::Up);
                } else {
                    self.grid = merge_row_vertical(self, Direction::Up);
                    self.spawn_tile();
                    self.update_highest_num();
                }
            }
            Direction::Down => {
                let new = merge_row_vertical(self, Direction::Down);

                if self.grid == new && !grid.contains(&0) {
                    self.game_over = true;
                    self.current_screen = Screen::GameOver;
                } else if self.grid == new {
                    self.grid = merge_row_vertical(self, Direction::Down);
                } else {
                    self.grid = merge_row_vertical(self, Direction::Down);
                    self.spawn_tile();
                    self.update_highest_num();
                }
            }
            Direction::Left => {
                let mut cells = self.grid.cells;

                for row in &mut cells {
                    *row = merge_row_horizontal(self, *row, Direction::Left)
                }

                if self.grid.cells == cells && !grid.contains(&0) {
                    self.game_over = true;
                    self.current_screen = Screen::GameOver;
                } else if self.grid.cells == cells {
                    self.grid.cells = cells;
                } else {
                    self.grid.cells = cells;
                    self.spawn_tile();
                    self.update_highest_num();
                }
            }
            Direction::Right => {
                let mut cells = self.grid.cells;

                for row in &mut cells {
                    *row = merge_row_horizontal(self, *row, Direction::Right)
                }

                if self.grid.cells == cells && !grid.contains(&0) {
                    self.game_over = true;
                    self.current_screen = Screen::GameOver;
                } else if self.grid.cells == cells {
                    self.grid.cells = cells;
                } else {
                    self.grid.cells = cells;
                    self.spawn_tile();
                    self.update_highest_num();
                }
            }
        }
    }

    /// Takes in self and mutates it to add a 2 or a 4, with the program weighting more to the 2
    /// The new tile is spawned in a random 0 location after the move has happened, based on the
    /// coordinates of 0 that we save during iteration
    pub fn spawn_tile(&mut self) {
        let nums = [2, 4];

        let mut rng = rng();
        let random_chance = Bernoulli::new(0.70).unwrap();
        let rand_selected = if random_chance.sample(&mut rng) {
            nums[0]
        } else {
            nums[1]
        };

        let mut zero_coordinates: Vec<(usize, usize)> = Vec::new();

        for (row_index, row) in self.grid.cells.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                if *col == 0 {
                    zero_coordinates.push((row_index, col_index));
                }
            }
        }

        let rand_coordinate = random_range(0..=zero_coordinates.len() - 1);
        let row = zero_coordinates[rand_coordinate].0;
        let col = zero_coordinates[rand_coordinate].1;

        self.grid.cells[row][col] = rand_selected;
    }

    pub fn update_highest_num(&mut self) {
        let cells = self.grid.cells;

        let max = cells.iter().flatten().copied().max().unwrap_or(0);

        if max > self.highest_num as u32 {
            self.highest_num = max as u8;
        }
    }

    pub fn toggle_scores(&mut self) {
        self.showing_score = !self.showing_score;
        if self.current_screen == Screen::Scores {
            self.current_screen = Screen::Playing
        } else {
            self.current_screen = Screen::Scores
        };
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

/// If the numbers in 0 and 1 match then we need to merge, we use the reader and writer vars to keep
/// track of the item we need to mutate as to not accidentally merge a number twice
/// reader tracks what val we are reading from the nums vec, if there is a merge we skip past the
/// next index so we dont double merge the same num
/// writer goes contiguous through each index
fn merge_row_horizontal(app: &mut App, row: [u32; 4], direction: Direction) -> [u32; 4] {
    let mut nums: Vec<u32> = row.into_iter().filter(|&val| val != 0).collect();
    if direction == Direction::Right || direction == Direction::Down {
        nums.reverse();
    }
    let mut reader = 0;
    let mut writer = 0;

    let mut result = [0; 4];

    while reader < nums.len() {
        if reader + 1 < nums.len() && nums[reader] == nums[reader + 1] {
            result[writer] = nums[reader] * 2;
            app.score += nums[reader] * 2;
            reader += 2;
        } else {
            result[writer] = nums[reader];
            reader += 1;
        }
        writer += 1;
    }

    if direction == Direction::Right || direction == Direction::Down {
        result.reverse();
    }
    result
}

/// Go through the whole Grid item from App and map the current grid to columns
/// From there it's the same mutation as above where we mutate and reverse if needed
fn merge_row_vertical(app: &mut App, direction: Direction) -> Grid {
    let mut cells = [[0; 4]; 4];

    for (col_index, _) in cells.into_iter().enumerate() {
        let column = [
            app.grid.cells[0][col_index],
            app.grid.cells[1][col_index],
            app.grid.cells[2][col_index],
            app.grid.cells[3][col_index],
        ];

        let merged = merge_row_horizontal(app, column, direction.clone());

        for row_index in 0..4 {
            cells[row_index][col_index] = merged[row_index]
        }
    }

    Grid { cells }
}

pub fn read_scores_file(path: PathBuf) -> String {
    let contents = fs::read_to_string(path);

    match contents {
        Ok(c) => c,
        Err(_) => "You have no high scores saved yet".to_string(),
    }
}

pub fn write_scores_to_file(app: &mut App, path: PathBuf) -> Result<()> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        let mut file = File::create(&path)?;
        writeln!(file, "Date Score Highest Num Game Style")?;
    }

    let mut file = OpenOptions::new().append(true).open(path)?;

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

    writeln!(
        file,
        "{} {} {} {}",
        now, app.score, app.highest_num, app.game_style
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app() -> App {
        App {
            highest_num: 0,
            score: 0,
            game_over: false,
            showing_score: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [[0, 0, 0, 0], [0, 0, 2, 2], [0, 2, 0, 0], [0, 0, 0, 0]],
            },
            current_screen: Screen::Playing,
            game_style: GameStyle::Normal,
            chosen_game_style: true,
        }
    }

    fn build_app_ended() -> App {
        App {
            highest_num: 0,
            score: 0,
            game_over: false,
            showing_score: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [
                    [2048, 1024, 512, 256],
                    [128, 64, 32, 16],
                    [8, 4, 2, 2048],
                    [1024, 512, 256, 128],
                ],
            },
            current_screen: Screen::Playing,
            game_style: GameStyle::Normal,
            chosen_game_style: true,
        }
    }

    #[test]
    fn rows_merge_left() {
        let mut app = build_app();
        assert_eq!(
            merge_row_horizontal(&mut app, [0, 2, 4, 8], Direction::Left),
            [2, 4, 8, 0]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [2, 4, 8, 0], Direction::Left),
            [2, 4, 8, 0]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [2, 2, 4, 2], Direction::Left),
            [4, 4, 2, 0]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [0, 0, 0, 0], Direction::Left),
            [0, 0, 0, 0]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [2, 4, 4, 0], Direction::Left),
            [2, 8, 0, 0]
        );
    }

    #[test]
    fn rows_merge_right() {
        let mut app = build_app();
        assert_eq!(
            merge_row_horizontal(&mut app, [0, 2, 4, 8], Direction::Right),
            [0, 2, 4, 8]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [0, 2, 4, 0], Direction::Right),
            [0, 0, 2, 4]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [0, 0, 0, 0], Direction::Right),
            [0, 0, 0, 0]
        );
        assert_eq!(
            merge_row_horizontal(&mut app, [0, 2, 2, 0], Direction::Right),
            [0, 0, 0, 4]
        );
    }

    #[test]
    fn rows_move_up() {
        let mut app = build_app();
        assert_eq!(
            merge_row_vertical(&mut app, Direction::Up),
            Grid {
                cells: [[0, 2, 2, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
            }
        );
    }

    #[test]
    fn rows_move_down() {
        let mut app = build_app();
        assert_eq!(
            merge_row_vertical(&mut app, Direction::Down),
            Grid {
                cells: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 2, 2, 2]]
            }
        );
    }

    #[test]
    fn random_tile_spawned() {
        let mut app = build_app();
        app.move_nums(Direction::Up);
        let original = app.grid.cells;
        app.spawn_tile();
        let new = app.grid.cells;
        assert_ne!(original, new);
    }

    #[test]
    fn read_write_scores() {
        let mut app = build_app();
        let path = PathBuf::from("./scores_app_test.txt");
        let _ = fs::remove_file(&path);

        write_scores_to_file(&mut app, path.clone()).unwrap();
        let contents = read_scores_file(path.clone());

        let expected = format!(
            "Date Score Highest Num Game Style\n{} 0 0 normal\n",
            chrono::Local::now().format("%Y-%m-%d %H:%M"),
        );

        assert_eq!(contents, expected);

        fs::remove_file(&path).unwrap();

        let contents = read_scores_file(path);
        assert_eq!(contents, "You have no high scores saved yet".to_string());
    }

    #[test]
    fn game_ends() {
        let mut app = build_app_ended();
        app.move_nums(Direction::Up);
        assert_eq!(app.current_screen, Screen::GameOver);
    }
}
