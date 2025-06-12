mod cell;
mod cursor;
mod position;

use super::tetromino_move::TetrominoMove;
use cell::Cell;
use cursor::piece::Piece;
use cursor::piece::Shape;
use cursor::Cursor;
use macroquad::color::colors::*;
use macroquad::prelude::{
    clear_background, draw_rectangle, draw_rectangle_lines, rand, request_new_screen_size,
};
use position::Position;

type Row = Vec<Cell>;

pub struct Board {
    num_rows: i32,
    num_cols: i32,
    rows: Vec<Row>,
    cursor_start_position: Position,
    cursor: Cursor,
    next_shape_candidates: Vec<Shape>,
}

const CELL_SIZE: f32 = 40.;
const LINE_THICKNESS: f32 = 2.;

impl Board {
    pub fn new() -> Self {
        rand::srand(macroquad::miniquad::date::now() as _);
        let num_cols: i32 = 10;
        let num_rows: i32 = 20;
        let mut rows = vec![vec![Cell::new(); num_cols as usize]; num_rows as usize];
        let cursor_start_position = Position {
            x: (num_cols - 1) / 2,
            y: 0,
        };
        let cursor = Cursor {
            position: cursor_start_position,
            piece: Piece::new(Shape::O),
        };
        Self::set_state_of_cells_at_cursor(&cursor, &mut rows, cell::State::Cursor);
        let next_shape_candidates = vec![
            Shape::O,
            Shape::I,
            Shape::T,
            Shape::S,
            Shape::Z,
            Shape::J,
            Shape::L,
        ];
        Board {
            num_rows,
            num_cols,
            rows,
            cursor_start_position,
            cursor,
            next_shape_candidates,
        }
    }

    fn set_state_of_cells_at_cursor(cursor: &Cursor, rows: &mut Vec<Row>, state: cell::State) {
        cursor.get_point_positions().iter().for_each(|position| {
            Self::set_state(&mut rows[position.y as usize][position.x as usize], state)
        });
    }

    fn set_state(cell: &mut Cell, state: cell::State) {
        cell.state = state;
    }

    pub fn update(&mut self, tetromino_move: TetrominoMove) {
        let new_cursor = Self::calc_new_cursor_pos_and_orientation(&self.cursor, tetromino_move);

        if self.fits_on_board(&new_cursor) {
            self.set_cell_states_at_cursor(cell::State::Empty);
            self.cursor = new_cursor;
            self.set_cell_states_at_cursor(cell::State::Cursor);
        } else {
            if tetromino_move == TetrominoMove::Down {
                self.dock_cursor_to_stack();
                //RemoveFullRowsFromStack();
                self.drop_new_piece();
            }
        }
    }

    #[rustfmt::skip]
    fn calc_new_cursor_pos_and_orientation(curr: &Cursor, tetromino_move: TetrominoMove) -> Cursor {
        let curr_pos = curr.position;
        let cur_x = curr_pos.x;
        let cur_y = curr_pos.y;
        match tetromino_move {
            TetrominoMove::Down  => return Cursor::from(curr, Position {x: cur_x    , y: cur_y + 1}),
            TetrominoMove::Left  => return Cursor::from(curr, Position {x: cur_x - 1, y: cur_y}),
            TetrominoMove::Right => return Cursor::from(curr, Position {x: cur_x + 1, y: cur_y}),
            TetrominoMove::RotateCW => return curr.clone(),
            TetrominoMove::RotateCCW => return curr.clone(),
        }
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
        let (w, h) = (&self.num_cols, &self.num_rows);
        return pos.x < 0 || pos.x >= *w || pos.y < 0 || pos.y >= *h;
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
                Self::set_state(
                    &mut self.rows[position.y as usize][position.x as usize],
                    cell::State::Stack,
                )
            });
    }

    fn drop_new_piece(&mut self) {
        let shape =
            self.next_shape_candidates[rand::gen_range(0, self.next_shape_candidates.len())];
        self.cursor = Cursor {
            position: self.cursor_start_position.clone(),
            piece: Piece::new(shape),
        };
        self.set_cell_states_at_cursor(cell::State::Cursor);
    }

    fn set_cell_states_at_cursor(&mut self, state: cell::State) {
        Self::set_state_of_cells_at_cursor(&self.cursor, &mut self.rows, state);
    }

    pub fn draw(&self) {
        request_new_screen_size(
            CELL_SIZE * self.num_cols as f32,
            CELL_SIZE * self.num_rows as f32,
        );

        clear_background(LIGHTGRAY);

        for (y, row) in self.rows.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                draw_cell(&cell.state, x, y);
            }
        }
    }
}

fn draw_cell(state: &cell::State, x: usize, y: usize) {
    let color = match state {
        cell::State::Empty => LIGHTGRAY,
        cell::State::Cursor => BROWN,
        cell::State::Stack => DARKGRAY,
    };

    draw_rectangle_lines(
        x as f32 * CELL_SIZE,
        y as f32 * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE,
        LINE_THICKNESS,
        GRAY,
    );

    draw_rectangle(
        x as f32 * CELL_SIZE + LINE_THICKNESS / 2.,
        y as f32 * CELL_SIZE + LINE_THICKNESS / 2.,
        CELL_SIZE - LINE_THICKNESS,
        CELL_SIZE - LINE_THICKNESS,
        color,
    );
}
