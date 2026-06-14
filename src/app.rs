use crate::grid::Grid;

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

    pub fn move_left(&mut self) {
        for row in &mut self.grid.cells {
            *row = merge_row_horizontal(*row, "left".to_string())
        }
    }

    pub fn move_right(&mut self) {
        for row in &mut self.grid.cells {
            *row = merge_row_horizontal(*row, "right".to_string())
        }
    }

    pub fn move_up(&mut self) {
        todo!()
    }

    pub fn move_down(&mut self) {
        todo!()
    }

    /// Score is calculated by the addition of current score + sum of any merged values
    pub fn calculate_score(&mut self) {
        todo!()
    }

    /// TODO: need to wire up scores to be saved in a .txt as "date score highest_num"
    /// access file and show in popup sorted by score
    pub fn show_scores(&mut self) {
        todo!()
    }

    /// TODO: calc open fields, randonmize 2 or 4, and popup in rand open
    pub fn spawn_tile(&mut self) {
        todo!()
    }

    // NOTE: dev only remove before release
    pub fn full_tiles(&mut self) {
        self.grid.cells = [[2, 4, 2, 2], [8, 16, 2, 4], [2, 2, 0, 0], [0, 0, 4, 2]]
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}

fn merge_row_horizontal(row: [u32; 4], direction: String) -> [u32; 4] {
    let mut nums: Vec<u32> = row.into_iter().filter(|&val| val != 0).collect();
    if direction == "right" {
        nums.reverse();
    }
    // reader tracks what val we are reading from the nums vec, if there is a merge we skip past the
    // next index so we dont double merge the same num
    let mut reader = 0;
    let mut writer = 0;

    let mut result = [0; 4];

    while reader < nums.len() {
        if reader + 1 < nums.len() && nums[reader] == nums[reader + 1] {
            result[writer] = nums[reader] * 2;
            reader += 2;
        } else {
            result[writer] = nums[reader];
            reader += 1;
        }
        writer += 1;
    }

    if direction == "right" {
        result.reverse();
    }
    result
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
        app.move_left();
        assert_eq!(
            app.grid.cells,
            [[2, 4, 4, 0], [8, 16, 2, 4], [4, 0, 0, 0], [4, 2, 0, 0]]
        );

        app.move_right();
        assert_eq!(
            app.grid.cells,
            [[0, 0, 2, 8], [8, 16, 2, 4], [0, 0, 0, 4], [0, 0, 4, 2]]
        );

        app.move_left();
        assert_eq!(
            app.grid.cells,
            [[2, 8, 0, 0], [8, 16, 2, 4], [4, 0, 0, 0], [4, 2, 0, 0]]
        );

        app.move_right();
        assert_eq!(
            app.grid.cells,
            [[0, 0, 2, 8], [8, 16, 2, 4], [0, 0, 0, 4], [0, 0, 4, 2]]
        );
    }

    #[test]
    fn rows_merge_left() {
        assert_eq!(
            merge_row_horizontal([0, 2, 4, 8], "left".to_string()),
            [2, 4, 8, 0]
        );
        assert_eq!(
            merge_row_horizontal([2, 4, 8, 0], "left".to_string()),
            [2, 4, 8, 0]
        );
        assert_eq!(
            merge_row_horizontal([2, 2, 4, 2], "left".to_string()),
            [4, 4, 2, 0]
        );
        assert_eq!(
            merge_row_horizontal([0, 0, 0, 0], "left".to_string()),
            [0, 0, 0, 0]
        );
        assert_eq!(
            merge_row_horizontal([2, 4, 4, 0], "left".to_string()),
            [2, 8, 0, 0]
        );
    }

    #[test]
    fn rows_move_left() {
        let mut app = build_app_default();
        app.move_left();
        assert_eq!(
            app.grid.cells,
            [[0, 0, 0, 0], [4, 0, 0, 0], [2, 0, 0, 0], [0, 0, 0, 0]]
        );

        let mut app = build_app_full();
        app.move_left();
        assert_eq!(
            app.grid.cells,
            [[2, 4, 4, 0], [8, 16, 2, 4], [4, 0, 0, 0], [4, 2, 0, 0]]
        );
    }

    #[test]
    fn rows_merge_right() {
        assert_eq!(
            merge_row_horizontal([0, 2, 4, 8], "right".to_string()),
            [0, 2, 4, 8]
        );
        assert_eq!(
            merge_row_horizontal([0, 2, 4, 0], "right".to_string()),
            [0, 0, 2, 4]
        );
        assert_eq!(
            merge_row_horizontal([0, 0, 0, 0], "right".to_string()),
            [0, 0, 0, 0]
        );
        assert_eq!(
            merge_row_horizontal([0, 2, 2, 0], "right".to_string()),
            [0, 0, 0, 4]
        );
    }

    #[test]
    fn rows_move_right() {
        let mut app = build_app_default();
        app.move_right();

        assert_eq!(
            app.grid.cells,
            [[0, 0, 0, 0], [0, 0, 0, 4], [0, 0, 0, 2], [0, 0, 0, 0]]
        );

        let mut app = build_app_full();
        app.move_right();
        assert_eq!(
            app.grid.cells,
            [[0, 2, 4, 4], [8, 16, 2, 4], [0, 0, 0, 4], [0, 0, 4, 2]]
        );
    }

    // #[test]
    // fn rows_merge_up() {
    //     assert_eq!(merge_row_right([0, 2, 4, 8]), [0, 2, 4, 8]);
    //     assert_eq!(merge_row_right([0, 2, 4, 0]), [0, 0, 2, 4]);
    //     assert_eq!(merge_row_right([0, 0, 0, 0]), [0, 0, 0, 0]);
    // }

    // #[test]
    // fn rows_move_up() {
    //     let mut app = build_app();
    //     app.move_right();
    //
    //     assert_eq!(
    //         app.grid.cells,
    //         [[0, 2, 2, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]]
    //     );
    // }

    // #[test]
    // fn rows_merge_down() {
    //     assert_eq!(merge_row_right([0, 2, 4, 8]), [0, 2, 4, 8]);
    //     assert_eq!(merge_row_right([0, 2, 4, 0]), [0, 0, 2, 4]);
    //     assert_eq!(merge_row_right([0, 0, 0, 0]), [0, 0, 0, 0]);
    // }

    // #[test]
    // fn rows_move_down() {
    //     let mut app = build_app();
    //     app.move_right();
    //
    //     assert_eq!(
    //         app.grid.cells,
    //         [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 2, 2, 0]]
    //     );
    // }
}
