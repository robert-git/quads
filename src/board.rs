pub mod cell;
pub mod cursor;
pub mod position;

use super::tetromino_move::TetrominoMove;
use super::user_move::UserMove;
use cell::Cell;
use cursor::piece::Shape;
use cursor::Cursor;
use macroquad::prelude::rand;
use position::Position;
use std::collections::VecDeque;
use std::fs::{metadata, File};
use std::io::{self, BufRead, Write};

pub type Row = Vec<Cell>;

pub struct Board {
    num_visible_rows: usize,
    num_total_rows: usize,
    num_cols: usize,
    rows: Vec<Row>,
    cursor_start_position: Position,
    cursor_queue: VecDeque<Cursor>,
    cursor: Cursor,
    next_shape_candidates: Vec<Shape>,
    score: i32,
    high_score: i32,
    row_removal_animation_is_pending: bool,
    rows_just_before_removal_of_full_rows: Vec<Row>,
}

const NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS: usize = 4;
type ToppedOut = bool;
type NumRowsClearedThisUpdate = usize;

impl Board {
    // Construction
    pub fn new() -> Self {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        rand::srand(macroquad::miniquad::date::now() as _);
        let num_visible_rows: usize = 20;
        let num_total_rows = num_visible_rows + NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS;
        let num_cols: usize = 10;

        let mut rows = vec![vec![Cell::new(); num_cols]; num_total_rows];

        let next_shape_candidates = vec![
            Shape::O,
            Shape::I,
            Shape::T,
            Shape::S,
            Shape::Z,
            Shape::J,
            Shape::L,
        ];

        let cursor_start_position = Position {
            #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
            x: (num_cols as i32 - 1) / 2,
            y: 0,
        };

        let mut cursor_queue = VecDeque::new();
        cursor_queue.push_back(Cursor::from_random_shape_in_list(
            &next_shape_candidates,
            cursor_start_position,
        ));
        cursor_queue.push_back(Cursor::from_random_shape_in_list(
            &next_shape_candidates,
            cursor_start_position,
        ));

        let cursor = cursor_queue.pop_front().unwrap();
        set_state_of_cells_at_cursor(&cursor, &mut rows, cell::State::Cursor);

        let rows_just_before_removal_of_full_rows = rows.clone();

        Board {
            num_visible_rows,
            num_total_rows,
            num_cols,
            rows,
            cursor_start_position,
            cursor,
            cursor_queue,
            next_shape_candidates,
            score: 0,
            high_score: read_high_score_from_file(),
            row_removal_animation_is_pending: false,
            rows_just_before_removal_of_full_rows,
        }
    }
}

impl Board {
    #[must_use]
    pub fn update(
        &mut self,
        tetromino_move: TetrominoMove,
    ) -> (ToppedOut, NumRowsClearedThisUpdate) {
        let mut topped_out: ToppedOut = false;
        let mut rows_cleared_this_update = 0;

        let hard_drop_y = self.calc_hard_drop_y(&self.cursor);

        let new_cursor =
            calc_new_cursor_pos_and_orientation(&self.cursor, tetromino_move, hard_drop_y);

        if self.fits_on_board(&new_cursor) {
            self.set_cell_states_at_cursor(cell::State::Empty);
            self.cursor = new_cursor;
            self.set_cell_states_at_cursor(cell::State::Cursor);
            match tetromino_move {
                TetrominoMove::UM(UserMove::SoftDown) => self.increment_score_by(1),
                TetrominoMove::UM(UserMove::HardDown) => {
                    self.increment_score_by(12);
                    let (new_topped_out, rows_cleared) = self.run_docking_sequence();
                    topped_out = new_topped_out;
                    rows_cleared_this_update = rows_cleared;
                }
                _ => (),
            }
        } else if tetromino_move == TetrominoMove::AutoDown
            || tetromino_move == TetrominoMove::UM(UserMove::SoftDown)
        {
            let (new_topped_out, rows_cleared) = self.run_docking_sequence();
            topped_out = new_topped_out;
            rows_cleared_this_update = rows_cleared;
        }

        (topped_out, rows_cleared_this_update)
    }

    fn calc_hard_drop_y(&self, cursor: &Cursor) -> i32 {
        let mut hard_drop_y = cursor.position.y;
        let mut point_positions = cursor.get_point_positions();

        while self.cursor_cells_fit_on_board(&point_positions) {
            increment_ys(&mut point_positions);
            hard_drop_y += 1;
        }
        hard_drop_y - 1
    }

    fn run_docking_sequence(&mut self) -> (ToppedOut, NumRowsClearedThisUpdate) {
        self.dock_cursor_to_stack();
        self.rows_just_before_removal_of_full_rows
            .clone_from(&self.rows);
        let num_rows_cleared = self.remove_full_rows_from_stack();
        let topped_out = self.stack_height() >= self.num_visible_rows;
        self.drop_new_piece();
        (topped_out, num_rows_cleared)
    }

    fn fits_on_board(&self, cursor: &Cursor) -> bool {
        let point_positions = cursor.get_point_positions();
        self.cursor_cells_fit_on_board(&point_positions)
    }

    fn cursor_cells_fit_on_board(&self, cursor_cell_positions: &[Position]) -> bool {
        if self.any_is_out_of_bounds(cursor_cell_positions) {
            return false;
        }
        self.all_not_occupied_by_stack(cursor_cell_positions)
    }

    fn any_is_out_of_bounds(&self, positions: &[Position]) -> bool {
        return positions.iter().any(|&pos| self.is_out_of_bounds(pos));
    }

    fn is_out_of_bounds(&self, pos: Position) -> bool {
        #[allow(clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
        let (w, h) = (self.num_cols as i32, self.num_total_rows as i32);
        !(0..w).contains(&pos.x) || !(0..h).contains(&pos.y)
    }

    fn all_not_occupied_by_stack(&self, positions: &[Position]) -> bool {
        #[allow(clippy::cast_sign_loss)]
        return positions
            .iter()
            .all(|&pos| self.rows[pos.y as usize][pos.x as usize].state != cell::State::Stack);
    }

    fn dock_cursor_to_stack(&mut self) {
        self.set_cell_states_at_cursor(cell::State::Stack);
    }

    fn remove_full_rows_from_stack(&mut self) -> NumRowsClearedThisUpdate {
        let orig_num_rows = self.rows.len();
        self.rows.retain(is_not_a_full_row);
        let num_removed_rows = orig_num_rows - self.rows.len();
        let new_rows = vec![vec![Cell::new(); self.num_cols]; num_removed_rows];
        self.rows.splice(0..0, new_rows);
        self.increment_score_by(get_points(num_removed_rows));
        if num_removed_rows > 0 {
            self.row_removal_animation_is_pending = true;
        }
        num_removed_rows
    }

    fn stack_height(&self) -> usize {
        let opt_index_highest_stack_row = self.rows.iter().position(contains_any_stack_cell);

        if let Some(index_highest_stack_row) = opt_index_highest_stack_row {
            self.num_total_rows - index_highest_stack_row
        } else {
            0
        }
    }

    fn drop_new_piece(&mut self) {
        self.cursor_queue
            .push_back(Cursor::from_random_shape_in_list(
                &self.next_shape_candidates,
                self.cursor_start_position,
            ));
        self.cursor = self.cursor_queue.pop_front().unwrap();
        self.set_cell_states_at_cursor(cell::State::Cursor);
    }

    fn set_cell_states_at_cursor(&mut self, state: cell::State) {
        set_state_of_cells_at_cursor(&self.cursor, &mut self.rows, state);
    }

    pub fn num_cols(&self) -> usize {
        self.num_cols
    }

    pub fn visible_rows(&self) -> &[Row] {
        &self.rows[NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS..]
    }

    pub fn visible_rows_just_before_removal_of_full_rows(&self) -> &[Row] {
        &self.rows_just_before_removal_of_full_rows[NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS..]
    }

    pub fn num_hidden_rows(&self) -> usize {
        self.num_total_rows - self.num_visible_rows
    }

    pub fn ghost_cursor_positions(&self) -> Vec<Position> {
        let hard_drop_y = self.calc_hard_drop_y(&self.cursor);
        let ghost_cursor = self.cursor.offset_copy(Position {
            x: self.cursor.position.x,
            y: hard_drop_y,
        });
        ghost_cursor.get_point_positions()
    }

    pub fn next_piece(&self) -> &cursor::piece::Piece {
        &self.cursor_queue.front().unwrap().piece
    }
}

impl Board {
    // Scoring
    fn increment_score_by(&mut self, increment_amount: i32) {
        self.score += increment_amount;
        self.high_score = self.score.max(self.high_score);
        write_high_score_to_file(self.high_score);
    }

    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn high_score(&self) -> i32 {
        self.high_score
    }

    pub fn row_removal_animation_is_pending(&self) -> bool {
        self.row_removal_animation_is_pending
    }

    pub fn set_row_removal_animation_is_pending_to_false(&mut self) {
        self.row_removal_animation_is_pending = false;
    }
}

const FILENAME_HIGH_SCORE: &str = "high_score.txt";

fn read_high_score_from_file() -> i32 {
    if metadata(FILENAME_HIGH_SCORE).is_ok() {
        if let Ok(file) = File::open(FILENAME_HIGH_SCORE) {
            let reader = io::BufReader::new(file);

            // Read the first line from the file
            if let Some(Ok(line)) = reader.lines().next() {
                // Attempt to parse the line as an i32
                if let Ok(number) = line.trim().parse::<i32>() {
                    return number;
                }
            }
        }
    }

    // Return 0 if the file doesn't exist or if parsing fails
    0
}

fn write_high_score_to_file(high_score: i32) {
    // Attempt to open the file in write-only mode, creating it if it doesn't exist
    let mut file = match File::create(FILENAME_HIGH_SCORE) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create or open file: {e}");
            return; // Exit the function if file opening fails
        }
    };

    // Attempt to write the number to the file
    if let Err(e) = writeln!(file, "{high_score}") {
        eprintln!("Failed to write to file: {e}");
    } else {
        println!("Number {high_score} written to {FILENAME_HIGH_SCORE}");
    }
}

fn increment_ys(positions: &mut [Position]) {
    positions.iter_mut().for_each(|pos| pos.y += 1);
}

#[rustfmt::skip]
fn calc_new_cursor_pos_and_orientation(curr: &Cursor, tetromino_move: TetrominoMove, hard_drop_y: i32) -> Cursor {
    let curr_pos = curr.position;
    let cur_x = curr_pos.x;
    let cur_y = curr_pos.y;
    match tetromino_move {
        TetrominoMove::AutoDown | TetrominoMove::UM(UserMove::SoftDown) => {
            curr.offset_copy(Position {x: cur_x, y: cur_y + 1,})
        }
        TetrominoMove::UM(UserMove::HardDown) => {
            curr.offset_copy(Position {x: cur_x, y: hard_drop_y,})
        }
        TetrominoMove::UM(UserMove::Left) => {
            curr.offset_copy(Position {x: cur_x - 1, y: cur_y,})
        }
        TetrominoMove::UM(UserMove::Right) => {
            curr.offset_copy(Position {x: cur_x + 1,y: cur_y,})
        }
        TetrominoMove::UM(UserMove::RotateCW) => curr.rotate_cw_copy(),
        TetrominoMove::UM(UserMove::RotateCCW) => curr.rotate_ccw_copy(),
    }
}

fn set_state_of_cells_at_cursor(cursor: &Cursor, rows: &mut [Row], state: cell::State) {
    #[allow(clippy::cast_sign_loss)]
    cursor
        .get_point_positions()
        .iter()
        .for_each(|position| set_state(&mut rows[position.y as usize][position.x as usize], state));
}

fn set_state(cell: &mut Cell, state: cell::State) {
    cell.state = state;
}

fn is_not_a_full_row(row: &Row) -> bool {
    !is_a_full_row(row)
}

fn is_a_full_row(row: &Row) -> bool {
    return row.iter().all(|&cell| cell.state == cell::State::Stack);
}

fn contains_any_stack_cell(row: &Row) -> bool {
    return row.iter().any(|&cell| cell.state == cell::State::Stack);
}

fn get_points(num_rows_removed: usize) -> i32 {
    match num_rows_removed {
        1 => 100,
        2 => 300,
        3 => 500,
        4 => 800,
        _ => 0,
    }
}
