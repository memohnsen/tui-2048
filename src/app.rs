use rand::random_range;

use crate::ui::Grid;

pub struct App {
    pub highest_num: u8,
    pub score: u32,
    pub game_over: bool,
    pub high_score: u32,
    pub exit: bool,
    pub grid: Grid,
    pub current_screen: Screen,
}

pub enum Screen {
    Playing,
    GameOver,
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

    // TODO: Bug - when at an edge and unable to move any cells that direction currently a move in
    // that dir is still allowed
    pub fn spawn_tile(&mut self) {
        let nums = [2, 4];
        let random_index = random_range(0..=1);
        let rand_selected = nums[random_index];

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

    /// TODO: need to wire up scores to be saved in a .txt as "date score highest_num"
    /// access file and show in popup sorted by score
    pub fn show_scores(&mut self) {
        todo!()
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

fn merge_row_horizontal(app: &mut App, row: [u32; 4], direction: Direction) -> [u32; 4] {
    let mut nums: Vec<u32> = row.into_iter().filter(|&val| val != 0).collect();
    if direction == Direction::Right || direction == Direction::Down {
        nums.reverse();
    }
    // reader tracks what val we are reading from the nums vec, if there is a merge we skip past the
    // next index so we dont double merge the same num
    let mut reader = 0;
    // writer goes contiguous through each index
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

#[cfg(test)]
mod tests {
    use super::*;

    fn build_app_default() -> App {
        App {
            highest_num: 0,
            score: 0,
            game_over: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [[0, 0, 0, 0], [0, 0, 2, 2], [0, 2, 0, 0], [0, 0, 0, 0]],
            },
            current_screen: Screen::Playing,
        }
    }

    fn build_app_full() -> App {
        App {
            highest_num: 0,
            score: 0,
            game_over: false,
            high_score: 0,
            exit: false,
            grid: Grid {
                cells: [[2, 4, 2, 2], [8, 16, 2, 4], [2, 2, 0, 0], [0, 0, 4, 2]],
            },
            current_screen: Screen::Playing,
        }
    }

    #[test]
    fn full_game_play() {
        let mut app = build_app_full();
        app.move_nums(Direction::Left);
        assert_eq!(
            app.grid.cells,
            [[2, 4, 4, 0], [8, 16, 2, 4], [4, 0, 0, 0], [4, 2, 0, 0]]
        );

        app.move_nums(Direction::Right);
        assert_eq!(
            app.grid.cells,
            [[0, 0, 2, 8], [8, 16, 2, 4], [0, 0, 0, 4], [0, 0, 4, 2]]
        );

        app.move_nums(Direction::Left);
        assert_eq!(
            app.grid.cells,
            [[2, 8, 0, 0], [8, 16, 2, 4], [4, 0, 0, 0], [4, 2, 0, 0]]
        );

        app.move_nums(Direction::Right);
        assert_eq!(
            app.grid.cells,
            [[0, 0, 2, 8], [8, 16, 2, 4], [0, 0, 0, 4], [0, 0, 4, 2]]
        );

        app.move_nums(Direction::Up);
        assert_eq!(
            app.grid.cells,
            [[8, 16, 4, 8], [0, 0, 4, 8], [0, 0, 0, 2], [0, 0, 0, 0]]
        );

        app.move_nums(Direction::Down);
        assert_eq!(
            app.grid.cells,
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 16], [8, 16, 8, 2]]
        );

        app.move_nums(Direction::Up);
        assert_eq!(
            app.grid.cells,
            [[8, 16, 8, 16], [0, 0, 0, 2], [0, 0, 0, 0], [0, 0, 0, 0]]
        );

        app.move_nums(Direction::Down);
        assert_eq!(
            app.grid.cells,
            [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 16], [8, 16, 8, 2]]
        );
    }

    #[test]
    fn rows_merge_left() {
        let mut app = build_app_default();
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
    fn rows_move_left() {
        let mut app = build_app_default();
        app.move_nums(Direction::Left);
        assert_eq!(
            app.grid.cells,
            [[0, 0, 0, 0], [4, 0, 0, 0], [2, 0, 0, 0], [0, 0, 0, 0]]
        );

        let mut app = build_app_full();
        app.move_nums(Direction::Left);
        assert_eq!(
            app.grid.cells,
            [[2, 4, 4, 0], [8, 16, 2, 4], [4, 0, 0, 0], [4, 2, 0, 0]]
        );
    }

    #[test]
    fn rows_merge_right() {
        let mut app = build_app_default();
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
    fn rows_move_right() {
        let mut app = build_app_default();
        app.move_nums(Direction::Right);

        assert_eq!(
            app.grid.cells,
            [[0, 0, 0, 0], [0, 0, 0, 4], [0, 0, 0, 2], [0, 0, 0, 0]]
        );

        let mut app = build_app_full();
        app.move_nums(Direction::Right);
        assert_eq!(
            app.grid.cells,
            [[0, 2, 4, 4], [8, 16, 2, 4], [0, 0, 0, 4], [0, 0, 4, 2]]
        );
    }

    #[test]
    fn rows_move_up() {
        let mut app = build_app_default();

        assert_eq!(
            merge_row_vertical(&mut app, Direction::Up),
            Grid {
                cells: [[0, 2, 2, 2], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
            }
        );
    }

    #[test]
    fn rows_move_down() {
        let mut app = build_app_default();

        assert_eq!(
            merge_row_vertical(&mut app, Direction::Down),
            Grid {
                cells: [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 2, 2, 2]]
            }
        );
    }
}
