mod cell;
mod cursor;
mod position;

use super::tetromino_move::TetrominoMove;
use cell::Cell;
use cursor::piece::Shape;
use cursor::Cursor;
use macroquad::color::colors::*;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, rand};
use position::Position;
use std::collections::VecDeque;

type Row = Vec<Cell>;

pub struct Board {
    num_visible_rows: usize,
    num_total_rows: usize,
    num_cols: usize,
    cell_size: f32,
    rows: Vec<Row>,
    cursor_start_position: Position,
    cursor_queue: VecDeque<Cursor>,
    cursor: Cursor,
    next_shape_candidates: Vec<Shape>,
}

const LINE_THICKNESS: f32 = 2.0;
const NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS: usize = 4;
type ToppedOut = bool;

impl Board {
    pub fn new(max_width: f32, max_height: f32) -> Self {
        rand::srand(macroquad::miniquad::date::now() as _);
        let num_visible_rows: usize = 20;
        let num_total_rows = num_visible_rows + NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS;
        let num_cols: usize = 10;

        let cell_size_from_width = max_width / num_cols as f32;
        let cell_size_from_height = max_height / num_visible_rows as f32;
        let cell_size = cell_size_from_width.min(cell_size_from_height);

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
            x: (num_cols as i32 - 1) / 2,
            y: 0,
        };

        let mut cursor_queue = VecDeque::new();
        cursor_queue.push_back(Cursor::random_shape(
            cursor_start_position.clone(),
            &next_shape_candidates,
        ));
        cursor_queue.push_back(Cursor::random_shape(
            cursor_start_position.clone(),
            &next_shape_candidates,
        ));

        let cursor = cursor_queue.pop_front().unwrap();
        set_state_of_cells_at_cursor(&cursor, &mut rows, cell::State::Cursor);

        Board {
            num_visible_rows,
            num_total_rows,
            num_cols,
            cell_size,
            rows,
            cursor_start_position,
            cursor,
            cursor_queue,
            next_shape_candidates,
        }
    }

    #[must_use]
    pub fn update(&mut self, tetromino_move: TetrominoMove) -> ToppedOut {
        let mut topped_out: ToppedOut = false;

        let new_cursor = calc_new_cursor_pos_and_orientation(&self.cursor, tetromino_move);

        if self.fits_on_board(&new_cursor) {
            self.set_cell_states_at_cursor(cell::State::Empty);
            self.cursor = new_cursor;
            self.set_cell_states_at_cursor(cell::State::Cursor);
        } else {
            if tetromino_move == TetrominoMove::Down {
                self.dock_cursor_to_stack();
                self.remove_full_rows_from_stack();
                topped_out = self.stack_height() >= self.num_visible_rows;
                self.drop_new_piece();
            }
        }
        topped_out
    }

    fn fits_on_board(&self, cursor: &Cursor) -> bool {
        let point_positions = cursor.get_point_positions();
        if self.any_is_out_of_bounds(&point_positions) {
            return false;
        }
        return self.all_not_occupied_by_stack(&point_positions);
    }

    fn any_is_out_of_bounds(&self, positions: &Vec<Position>) -> bool {
        return positions.iter().any(|&pos| self.is_out_of_bounds(&pos));
    }

    fn is_out_of_bounds(&self, pos: &Position) -> bool {
        let (w, h) = (self.num_cols as i32, self.num_total_rows as i32);
        return pos.x < 0 || pos.x >= w || pos.y < 0 || pos.y >= h;
    }

    fn all_not_occupied_by_stack(&self, positions: &Vec<Position>) -> bool {
        return positions
            .iter()
            .all(|&pos| self.rows[pos.y as usize][pos.x as usize].state != cell::State::Stack);
    }

    fn dock_cursor_to_stack(&mut self) {
        self.cursor
            .get_point_positions()
            .iter()
            .for_each(|position| {
                set_state(
                    &mut self.rows[position.y as usize][position.x as usize],
                    cell::State::Stack,
                )
            });
    }

    fn remove_full_rows_from_stack(&mut self) {
        let orig_num_rows = self.rows.len();
        self.rows.retain(|row| is_not_a_full_row(row));
        let num_removed_rows = orig_num_rows - self.rows.len();
        let new_rows = vec![vec![Cell::new(); self.num_cols]; num_removed_rows];
        self.rows.splice(0..0, new_rows);
    }

    fn stack_height(&self) -> usize {
        let opt_index_highest_stack_row = self
            .rows
            .iter()
            .position(|row| contains_any_stack_cell(&row));

        if opt_index_highest_stack_row.is_some() {
            self.num_total_rows - opt_index_highest_stack_row.unwrap()
        } else {
            0
        }
    }

    fn drop_new_piece(&mut self) {
        self.cursor_queue.push_back(Cursor::random_shape(
            self.cursor_start_position.clone(),
            &self.next_shape_candidates,
        ));
        self.cursor = self.cursor_queue.pop_front().unwrap();
        self.set_cell_states_at_cursor(cell::State::Cursor);
    }

    fn set_cell_states_at_cursor(&mut self, state: cell::State) {
        set_state_of_cells_at_cursor(&self.cursor, &mut self.rows, state);
    }

    pub fn draw(&self) {
        {
            let preview_base_col: usize = self.num_cols + 3;
            let preview_base_row: usize = 2;
            draw_preview_piece(
                self.cursor_queue.front().unwrap(),
                preview_base_col,
                preview_base_row,
                self.cell_size,
            );
        }

        for (y, row) in self.rows[NUM_HIDDEN_ROWS_ABOVE_VISIBLE_ROWS..]
            .iter()
            .enumerate()
        {
            for (x, cell) in row.iter().enumerate() {
                draw_cell(cell.state.clone(), x, y, self.cell_size);
            }
        }
    }
}

#[rustfmt::skip]
fn calc_new_cursor_pos_and_orientation(curr: &Cursor, tetromino_move: TetrominoMove) -> Cursor {
    let curr_pos = curr.position;
    let cur_x = curr_pos.x;
    let cur_y = curr_pos.y;
    match tetromino_move {
        TetrominoMove::Down  => return curr.offset_copy(Position {x: cur_x    , y: cur_y + 1}),
        TetrominoMove::Left  => return curr.offset_copy(Position {x: cur_x - 1, y: cur_y}),
        TetrominoMove::Right => return curr.offset_copy(Position {x: cur_x + 1, y: cur_y}),
        TetrominoMove::RotateCW  => return curr.rotate_cw_copy(),
        TetrominoMove::RotateCCW => return curr.rotate_ccw_copy(),
    }
}

fn set_state_of_cells_at_cursor(cursor: &Cursor, rows: &mut Vec<Row>, state: cell::State) {
    cursor
        .get_point_positions()
        .iter()
        .for_each(|position| set_state(&mut rows[position.y as usize][position.x as usize], state));
}

fn set_state(cell: &mut Cell, state: cell::State) {
    cell.state = state;
}

fn is_not_a_full_row(row: &Row) -> bool {
    return !is_a_full_row(&row);
}

fn is_a_full_row(row: &Row) -> bool {
    return row.iter().all(|&cell| cell.state == cell::State::Stack);
}

fn contains_any_stack_cell(row: &Row) -> bool {
    return row.iter().any(|&cell| cell.state == cell::State::Stack);
}

fn draw_preview_piece(cursor: &Cursor, base_col_idx: usize, base_row_idx: usize, cell_size: f32) {
    for &pos in cursor.piece.get_local_points().iter() {
        let cell_col_idx = (base_col_idx as i32 + pos.x) as usize;
        let cell_row_idx = (base_row_idx as i32 + pos.y) as usize;
        draw_cell(cell::State::Cursor, cell_col_idx, cell_row_idx, cell_size);
    }
}

fn draw_cell(state: cell::State, col_idx: usize, row_idx: usize, cell_size: f32) {
    #[rustfmt::skip]
    let outline_color = match state {
        cell::State::Empty  => Color::new(0.99, 0.99, 0.99, 1.00),
        cell::State::Cursor => BEIGE,
        cell::State::Stack  => GRAY,
    };

    #[rustfmt::skip]
    let fill_color = match state {
        cell::State::Empty  => WHITE,
        cell::State::Cursor => BROWN,
        cell::State::Stack  => DARKGRAY,
    };

    draw_rectangle_lines(
        col_idx as f32 * cell_size,
        row_idx as f32 * cell_size,
        cell_size,
        cell_size,
        LINE_THICKNESS,
        outline_color,
    );

    draw_rectangle(
        col_idx as f32 * cell_size + LINE_THICKNESS / 2.,
        row_idx as f32 * cell_size + LINE_THICKNESS / 2.,
        cell_size - LINE_THICKNESS,
        cell_size - LINE_THICKNESS,
        fill_color,
    );
}
