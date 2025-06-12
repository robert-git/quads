mod cell;
mod cursor;
mod position;

use super::tetromino_move::TetrominoMove;
use cell::Cell;
use cursor::piece::Piece;
use cursor::Cursor;
use macroquad::color::colors::*;
use macroquad::prelude::{
    clear_background, draw_rectangle, draw_rectangle_lines, request_new_screen_size,
};
use position::Position;

type Row = Vec<Cell>;

pub struct Board {
    num_rows: i32,
    num_cols: i32,
    rows: Vec<Row>,
    cursor_start_position: Position,
    cursor: Cursor,
}

const CELL_SIZE: f32 = 40.;
const LINE_THICKNESS: f32 = 2.;

impl Board {
    pub fn new() -> Self {
        let num_cols: i32 = 10;
        let num_rows: i32 = 20;
        let mut rows = vec![vec![Cell::new(); num_cols as usize]; num_rows as usize];
        let cursor_start_position = Position {
            x: (num_cols - 1) / 2,
            y: 0,
        };
        let cursor = Cursor {
            position: cursor_start_position,
            piece: Piece::new(),
        };
        Self::set_state_of_cell_at_cursor(&cursor, &mut rows, cell::State::Cursor);
        Board {
            num_rows,
            num_cols,
            rows,
            cursor_start_position,
            cursor,
        }
    }

    fn set_state_of_cell_at_cursor(cursor: &Cursor, rows: &mut Vec<Row>, state: cell::State) {
        Self::set_state(
            &mut rows[cursor.position.y as usize][cursor.position.x as usize],
            state,
        );
    }

    fn set_state(cell: &mut Cell, state: cell::State) {
        cell.state = state;
    }

    pub fn update(&mut self, tetromino_move: TetrominoMove) {
        let new_cursor = Self::calc_new_cursor_pos_and_orientation(&self.cursor, tetromino_move);

        if self.fits_on_board(&new_cursor) {
            self.set_cursor_state(cell::State::Empty);
            self.cursor = new_cursor;
            self.set_cursor_state(cell::State::Cursor);
        } else {
            if tetromino_move == TetrominoMove::Down {
                //DockCurrentPieceToStack();
                //RemoveFullRowsFromStack();
                //DropNewPiece();
            }
        }
        /*
                match tetromino_move {
                    TetrominoMove::Down => self.try_move_cursor_down(),
                    TetrominoMove::Left => self.try_move_cursor_left(),
                    TetrominoMove::Right => self.try_move_cursor_right(),
                    TetrominoMove::RotateCW => (),
                    TetrominoMove::RotateCCW => (),
                }
        */
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
        let points = cursor.get_points();
        if self.any_is_out_of_bounds(&points) {
            return false;
        }
        return self.all_not_occupied_by_stack(&points);
    }

    fn any_is_out_of_bounds(&self, positions: &Vec<Position>) -> bool {
        return false;
    }

    fn all_not_occupied_by_stack(&self, positions: &Vec<Position>) -> bool {
        return true;
    }

    fn try_move_cursor_down(&mut self) {
        self.move_cursor(|board: &mut Board| {
            // TODO: Placeholder logic, good enough for now until I implement the collision logic:
            if board.cursor.position.y < board.num_rows - 1 {
                board.cursor.position.y += 1;
            };
        });
    }

    fn try_move_cursor_left(&mut self) {
        self.move_cursor(|board: &mut Board| {
            // TODO: Placeholder logic, good enough for now until I implement the collision logic:
            if board.cursor.position.x > 0 {
                board.cursor.position.x -= 1;
            }
        });
    }

    fn try_move_cursor_right(&mut self) {
        self.move_cursor(|board: &mut Board| {
            // TODO: Placeholder logic, good enough for now until I implement the collision logic:
            if board.cursor.position.x < board.num_cols - 1 {
                board.cursor.position.x += 1;
            }
        });
    }

    fn move_cursor<UpdateCursorPosition>(&mut self, update_cursor_pos: UpdateCursorPosition)
    where
        UpdateCursorPosition: Fn(&mut Self),
    {
        self.set_cursor_state(cell::State::Empty);
        update_cursor_pos(self);
        self.set_cursor_state(cell::State::Cursor);
    }

    fn set_cursor_state(&mut self, state: cell::State) {
        Self::set_state_of_cell_at_cursor(&self.cursor, &mut self.rows, state);
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
