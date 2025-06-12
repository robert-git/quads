use super::board::cell;
use super::board::cursor::piece::Piece;
use super::board::Board;
use macroquad::color::colors::*;
use macroquad::color::Color;
use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, draw_text};

const LINE_THICKNESS: f32 = 2.0;

pub struct SizeInPixels {
    pub width: f32,
    pub height: f32,
}

pub fn draw(board: &Board, canvas_size: SizeInPixels) {
    let num_board_cols = board.num_cols();
    let visible_rows = board.visible_rows();

    let cell_size = calc_cell_size_in_pixels(canvas_size, num_board_cols, visible_rows.len());

    draw_preview_of_next_piece(board.next_piece(), num_board_cols, cell_size);

    draw_score(board.score(), num_board_cols, cell_size);

    for (y, row) in visible_rows.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            draw_cell(cell.state.clone(), x, y, cell_size);
        }
    }
}

fn calc_cell_size_in_pixels(
    canvas_size: SizeInPixels,
    num_board_cols: usize,
    num_visible_board_rows: usize,
) -> f32 {
    let cell_size_from_width = canvas_size.width / num_board_cols as f32;
    let cell_size_from_height = canvas_size.height / num_visible_board_rows as f32;
    cell_size_from_width.min(cell_size_from_height)
}

fn draw_score(score: i32, num_board_cols: usize, cell_size: f32) {
    let font_size = 30.;
    let spacer_cols_x = 2;
    let pixel_offset_x = (num_board_cols + spacer_cols_x) as f32 * cell_size;
    let pixel_offset_y = 40.0;
    draw_text(
        score.to_string().as_str(),
        pixel_offset_x,
        pixel_offset_y,
        font_size,
        DARKGRAY,
    );
}

fn draw_preview_of_next_piece(next_piece: &Piece, num_board_cols: usize, cell_size: f32) {
    let base_col_idx: usize = num_board_cols + 3;
    let base_row_idx: usize = 2;
    for &pos in next_piece.get_local_points().iter() {
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
