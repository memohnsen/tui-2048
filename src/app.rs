use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
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
}

#[derive(PartialEq)]
pub enum Screen {
    Playing,
    GameOver,
    Scores,
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
    }

    // TODO: Bug - when at an edge and unable to move any cells that direction currently a move in
    // that dir is still allowed
    pub fn move_nums(&mut self, direction: Direction) {
        match direction {
            Direction::Up => {
                self.grid = merge_row_vertical(self, Direction::Up);
                self.spawn_tile();
            }
            Direction::Down => {
                self.grid = merge_row_vertical(self, Direction::Down);
                self.spawn_tile();
            }
            Direction::Left => {
                let mut cells = self.grid.cells;

                for row in &mut cells {
                    *row = merge_row_horizontal(self, *row, Direction::Left)
                }

                self.grid.cells = cells;
                self.spawn_tile();
            }
            Direction::Right => {
                let mut cells = self.grid.cells;

                for row in &mut cells {
                    *row = merge_row_horizontal(self, *row, Direction::Right)
                }

                self.grid.cells = cells;
                self.spawn_tile();
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

        if zero_coordinates.is_empty() {
            self.game_over = true;
            return self.current_screen = Screen::GameOver;
        }

        let rand_coordinate = random_range(0..=zero_coordinates.len() - 1);
        let row = zero_coordinates[rand_coordinate].0;
        let col = zero_coordinates[rand_coordinate].1;

        self.grid.cells[row][col] = rand_selected;
    }

    // TODO: track highest_num

    pub fn toggle_scores(&mut self) {
        self.showing_score = !self.showing_score;
        if self.current_screen == Screen::Scores {
            self.current_screen = Screen::Playing
        } else {
            self.current_screen = Screen::Scores
        };
    }

    pub fn write_scores_to_file(&mut self) -> Result<()> {
        let home = std::env::var("HOME").unwrap_or("~".to_string());

        let mut path = PathBuf::from(home);
        path.push(".config/2048");

        fs::create_dir_all(&path)?;
        path.push("scores.txt");

        if !path.exists() {
            let mut file = File::create(&path)?;
            writeln!(file, "Date Score Highest Num")?;
        }

        let mut file = OpenOptions::new().append(true).open(path)?;

        let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();

        writeln!(file, "{} {} {}", now, self.score, self.highest_num)?;
        Ok(())
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

pub fn read_scores_file() -> String {
    let home = std::env::var("HOME").unwrap_or("~".to_string());

    let mut path = PathBuf::from(home);
    path.push(".config/2048/scores.txt");

    let contents = fs::read_to_string(path);

    match contents {
        Ok(c) => c,
        Err(_) => "You have no high scores saved yet".to_string(),
    }
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
}
