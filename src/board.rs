mod cell;
mod position;

use cell::Cell;
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
}

const cell_size: f32 = 40.;
const line_thickness: f32 = 2.;

impl Board {
    pub fn new() -> Self {
        let num_cols: i32 = 10;
        let num_rows: i32 = 20;
        let mut rows = vec![vec![Cell::new(); num_cols as usize]; num_rows as usize];
        let cursor_start_position = Position {
            x: (num_cols - 1) / 2,
            y: 0,
        };
        rows[cursor_start_position.y as usize][cursor_start_position.x as usize].state =
            cell::State::Cursor;
        Board {
            num_rows,
            num_cols,
            rows,
            cursor_start_position,
        }
    }

    pub fn update(&mut self, tetromino_move: &crate::tetromino_move::TetrominoMove) {}

    pub fn draw(&self) {
        request_new_screen_size(
            cell_size * self.num_cols as f32,
            cell_size * self.num_rows as f32,
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
        x as f32 * cell_size,
        y as f32 * cell_size,
        cell_size,
        cell_size,
        line_thickness,
        GRAY,
    );
    draw_rectangle(
        x as f32 * cell_size + line_thickness,
        y as f32 * cell_size + line_thickness,
        cell_size - 2 as f32 * line_thickness,
        cell_size - 2 as f32 * line_thickness,
        color,
    );
}
