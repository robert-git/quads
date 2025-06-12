mod cell;
mod cursor;
mod position;

use cell::Cell;
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
        };
        Self::set_state(
            &mut rows[cursor.position.y as usize][cursor.position.x as usize],
            cell::State::Cursor,
        );
        Board {
            num_rows,
            num_cols,
            rows,
            cursor_start_position,
            cursor,
        }
    }

    fn set_cursor_state(&mut self, cursor: &Cursor, state: cell::State) {
        Self::set_state(
            &mut self.rows[cursor.position.y as usize][cursor.position.x as usize],
            state,
        );
    }

    fn set_state(cell: &mut Cell, state: cell::State) {
        cell.state = state;
    }

    pub fn update(&mut self, tetromino_move: crate::tetromino_move::TetrominoMove) {
        use crate::tetromino_move::TetrominoMove;

        match tetromino_move {
            TetrominoMove::Down  => self.move_cursor_down(),
            TetrominoMove::Left  => self.move_cursor_left(),
            TetrominoMove::Right => self.move_cursor_right(),
            TetrominoMove::RotateCW => (),
            TetrominoMove::RotateCCW => (),
        }
    }

    fn move_cursor_down(&mut self) {
        self.move_cursor(|board: &mut Board| {
            // TODO: Placeholder logic, good enough for now until I implement the collision logic:
            if board.cursor.position.y < board.num_rows - 1 {
                board.cursor.position.y += 1;
            };
        });
    }

    fn move_cursor_left(&mut self) {
        self.move_cursor(|board: &mut Board| {
            // TODO: Placeholder logic, good enough for now until I implement the collision logic:
            if board.cursor.position.x > 0 {
                board.cursor.position.x -= 1;
            }
        });
    }

    fn move_cursor_right(&mut self) {
        self.move_cursor(|board: &mut Board| {
            // TODO: Placeholder logic, good enough for now until I implement the collision logic:
            if board.cursor.position.x < board.num_cols - 1 {
                board.cursor.position.x += 1;
            }
        });
    }

    fn move_cursor<UpdateCursorPosition>(&mut self, mut update_cursor_pos: UpdateCursorPosition)
    where
        UpdateCursorPosition: FnMut(&mut Self),
    {
        {
            let mut cell =
                &mut self.rows[self.cursor.position.y as usize][self.cursor.position.x as usize];
            Self::set_state(&mut cell, cell::State::Empty);
        }

        update_cursor_pos(self);

        let mut cell =
            &mut self.rows[self.cursor.position.y as usize][self.cursor.position.x as usize];
        Self::set_state(&mut cell, cell::State::Cursor);
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
