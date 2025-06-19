use super::board::cell;
use super::board::cursor;
use super::board::cursor::piece::Piece;
use super::board::position::Position;
use super::board::Board;
use super::board::Row;
use macroquad::color::colors::*;
use macroquad::color::Color;
use macroquad::prelude::{
    clear_background, draw_rectangle, draw_rectangle_lines, draw_text, measure_text, screen_height,
    screen_width,
};
use std::thread;

const LINE_THICKNESS: f32 = 2.0;

#[derive(Clone)]
pub struct SizeInPixels {
    pub width: f32,
    pub height: f32,
}

#[derive(PartialEq, Copy, Clone)]
enum CellDisplayState {
    Empty,
    Cursor,
    Stack,
    BeingRemoved,
}

type DisplayRow = Vec<CellDisplayState>;

#[derive(Clone)]
struct BoardState {
    num_cols: usize,
    latest_visible_rows: Vec<DisplayRow>,
    visible_rows_just_before_removal_of_full_rows: Vec<DisplayRow>,
    next_piece: cursor::piece::Piece,
    score: i32,
    ghost_cursor_positions: Vec<Position>,
    num_hidden_rows: usize,
}

pub struct Renderer {
    canvas_size: SizeInPixels,
    font_size: f32,
    drawing_row_removal_animation: bool,
    animation_frames_left_to_draw: i32,
    indices_of_full_rows_to_animate: Vec<usize>,
    animation_row: DisplayRow,
    board_state: Option<BoardState>,
    first_frame_post_animation: bool,
}

impl Renderer {
    pub fn new(canvas_size: &SizeInPixels) -> Self {
        let original_canvas_height = 800.0;
        let original_font_size = 30.0;
        Renderer {
            canvas_size: canvas_size.clone(),
            font_size: original_font_size * (canvas_size.height / original_canvas_height),
            drawing_row_removal_animation: false,
            animation_frames_left_to_draw: 0,
            indices_of_full_rows_to_animate: Vec::new(),
            animation_row: Vec::new(),
            board_state: None,
            first_frame_post_animation: false,
        }
    }

    pub fn draw(&mut self, board: &mut Board) {
        let board_state = get_board_state(board);
        let num_cols = board_state.num_cols;
        let num_frames_to_animate = num_cols as i32 / 2;
        let delay_between_animated_frames = std::time::Duration::from_millis(60);

        if board.row_removal_animation_is_pending() && !self.drawing_row_removal_animation {
            self.initialize_row_removal_animation(board_state.clone(), num_frames_to_animate);
        }

        if self.animation_frames_left_to_draw > 0 {
            let first_frame_of_current_animation =
                num_frames_to_animate == self.animation_frames_left_to_draw;

            self.draw_next_row_removal_animation_frame();

            if !first_frame_of_current_animation {
                thread::sleep(delay_between_animated_frames);
            }
        } else {
            self.draw_normal_non_animated_board_state(
                board,
                delay_between_animated_frames,
                &board_state,
            );
        }
    }

    pub fn drawing_row_removal_animation(&self) -> bool {
        self.drawing_row_removal_animation
    }

    fn initialize_row_removal_animation(
        &mut self,
        board_state: BoardState,
        num_frames_to_animate: i32,
    ) {
        let num_cols = board_state.num_cols;
        self.board_state = Some(board_state);
        self.drawing_row_removal_animation = true;
        self.animation_frames_left_to_draw = num_frames_to_animate;
        self.indices_of_full_rows_to_animate = get_indices_of_full_rows(
            &self
                .board_state
                .as_ref()
                .unwrap()
                .visible_rows_just_before_removal_of_full_rows,
        );
        self.animation_row = vec![CellDisplayState::BeingRemoved; num_cols];
    }

    fn draw_next_row_removal_animation_frame(&mut self) {
        self.first_frame_post_animation = true;
        // print_rows(
        //     &board_state.visible_rows_just_before_removal_of_full_rows,
        //     "Renderer before",
        // );
        // print_rows(&board_state.visible_rows, "Renderer after");
        println!("{:?}", self.indices_of_full_rows_to_animate);
        println!("anim row:");
        print_row(&self.animation_row);
        draw_helper(
            self.board_state.as_ref().unwrap(),
            DrawMode::AnimatingRowRemoval,
            &self.canvas_size,
            self.font_size,
        );

        make_next_frame_of_row_removal_animation(
            &mut self
                .board_state
                .as_mut()
                .unwrap()
                .visible_rows_just_before_removal_of_full_rows,
            &self.indices_of_full_rows_to_animate,
            &mut self.animation_row,
        );
        self.animation_frames_left_to_draw -= 1;
    }

    fn draw_normal_non_animated_board_state(
        &mut self,
        board: &mut Board,
        delay_between_animated_frames: std::time::Duration,
        board_state: &BoardState,
    ) {
        board.set_row_removal_animation_is_pending_to_false();

        self.drawing_row_removal_animation = false;

        if self.first_frame_post_animation {
            self.first_frame_post_animation = false;
            thread::sleep(delay_between_animated_frames);
        }

        draw_helper(
            board_state,
            DrawMode::NotAnimatingRowRemoval,
            &self.canvas_size,
            self.font_size,
        );
    }
}

fn print_rows(rows: &[DisplayRow], desc: &str) {
    println!("{desc}:");
    for row in rows.iter() {
        print_row(row);
    }
}

fn print_row(row: &DisplayRow) {
    print!("|");
    for cell in row.iter() {
        print_cell(&cell);
    }
    println!("|");
}

fn print_cell(cell_display_state: &CellDisplayState) {
    let ch = match cell_display_state {
        CellDisplayState::Empty => " ",
        CellDisplayState::Cursor => "*",
        CellDisplayState::Stack => "x",
        CellDisplayState::BeingRemoved => "y",
    };
    print!("{ch}");
}

fn get_board_state(board: &Board) -> BoardState {
    let num_cols = board.num_cols();
    let visible_rows = rows_to_display_rows(board.visible_rows().to_vec());
    let visible_rows_just_before_removal_of_full_rows = rows_to_display_rows(
        board
            .visible_rows_just_before_removal_of_full_rows()
            .to_vec(),
    );
    let next_piece = board.next_piece().clone();
    let score = board.score();
    let ghost_cursor_positions = board.ghost_cursor_positions();
    let num_hidden_rows = board.num_hidden_rows();
    BoardState {
        num_cols,
        latest_visible_rows: visible_rows,
        visible_rows_just_before_removal_of_full_rows,
        next_piece,
        score,
        ghost_cursor_positions,
        num_hidden_rows,
    }
}

fn rows_to_display_rows(src: Vec<Row>) -> Vec<DisplayRow> {
    src.into_iter().map(row_to_display_row).collect()
}

fn row_to_display_row(src: Row) -> DisplayRow {
    src.into_iter()
        .map(cell_to_cell_display_state)
        .collect()
}

fn cell_to_cell_display_state(src: cell::Cell) -> CellDisplayState {
    match src.state {
        cell::State::Empty => CellDisplayState::Empty,
        cell::State::Cursor => CellDisplayState::Cursor,
        cell::State::Stack => CellDisplayState::Stack,
    }
}

fn get_indices_of_full_rows(rows: &[DisplayRow]) -> Vec<usize> {
    rows.iter()
        .enumerate()
        .filter(|&(_, row)| is_full(row))
        .map(|(index, _)| index)
        .collect()
}

fn is_full(row: &DisplayRow) -> bool {
    return row
        .iter()
        .all(|&cell_display_state| cell_display_state == CellDisplayState::Stack);
}

enum DrawMode {
    NotAnimatingRowRemoval,
    AnimatingRowRemoval,
}

fn draw_helper(
    board_state: &BoardState,
    draw_mode: DrawMode,
    canvas_size: &SizeInPixels,
    font_size: f32,
) {
    let num_board_cols = board_state.num_cols;
    let visible_rows = match draw_mode {
        DrawMode::NotAnimatingRowRemoval => &board_state.latest_visible_rows,
        DrawMode::AnimatingRowRemoval => &board_state.visible_rows_just_before_removal_of_full_rows,
    };

    let cell_size = calc_cell_size_in_pixels(canvas_size, num_board_cols, visible_rows.len());

    draw_preview_of_next_piece(&board_state.next_piece, num_board_cols, cell_size);

    draw_score(board_state.score, num_board_cols, cell_size, font_size);

    for (y, row) in visible_rows.iter().enumerate() {
        for (x, cell_display_state) in row.iter().enumerate() {
            draw_cell(*cell_display_state, x, y, cell_size);
        }
    }

    if matches!(draw_mode, DrawMode::NotAnimatingRowRemoval) {
        draw_ghost_cursor(
            board_state.ghost_cursor_positions.clone(),
            board_state.num_hidden_rows,
            cell_size,
        );
    }
}

fn make_next_frame_of_row_removal_animation(
    rows: &mut Vec<DisplayRow>,
    indices_of_full_rows_to_animate: &Vec<usize>,
    animation_row: &mut DisplayRow,
) {
    enlarge_middle_gap(animation_row);

    print_rows(rows, "Before replacement");
    for &index in indices_of_full_rows_to_animate {
        (*rows)[index].clone_from(animation_row);
    }
    print_rows(rows, "After replacement");
}

fn enlarge_middle_gap(animation_row: &mut DisplayRow) {
    let len = animation_row.len();
    let i1 = {
        let opt_idx_of_1st_non_stack = animation_row.iter().position(|cell_display_state| {
            !matches!(cell_display_state, CellDisplayState::BeingRemoved)
        });

        if let Some(idx_of_1st_non_stack) = opt_idx_of_1st_non_stack {
            if idx_of_1st_non_stack == 0 {
                0
            } else {
                idx_of_1st_non_stack - 1
            }
        } else {
            len / 2
        }
    };
    let i2 = len - i1 - 1;
    animation_row[i1] = CellDisplayState::Empty;
    animation_row[i2] = CellDisplayState::Empty;
}

fn calc_cell_size_in_pixels(
    canvas_size: &SizeInPixels,
    num_board_cols: usize,
    num_visible_board_rows: usize,
) -> f32 {
    let cell_size_from_width = canvas_size.width / num_board_cols as f32;
    let cell_size_from_height = canvas_size.height / num_visible_board_rows as f32;
    cell_size_from_width.min(cell_size_from_height)
}

fn draw_score(score: i32, num_board_cols: usize, cell_size: f32, font_size: f32) {
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
        draw_cell(
            CellDisplayState::Cursor,
            cell_col_idx,
            cell_row_idx,
            cell_size,
        );
    }
}

fn draw_cell(cell_display_state: CellDisplayState, col_idx: usize, row_idx: usize, cell_size: f32) {
    #[rustfmt::skip]
    let outline_color = match cell_display_state {
        CellDisplayState::Empty        => Color::new(0.99, 0.99, 0.99, 1.00),
        CellDisplayState::Cursor       => BEIGE,
        CellDisplayState::Stack        => GRAY,
        CellDisplayState::BeingRemoved => LIME,
    };

    #[rustfmt::skip]
    let fill_color = match cell_display_state {
        CellDisplayState::Empty         => WHITE,
        CellDisplayState::Cursor        => BROWN,
        CellDisplayState::Stack         => DARKGRAY,
        CellDisplayState::BeingRemoved  => GREEN,
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

fn draw_ghost_cursor(
    ghost_cursor_cell_positions: Vec<Position>,
    num_hidden_board_rows: usize,
    cell_size: f32,
) {
    ghost_cursor_cell_positions
        .iter()
        .for_each(|pos| draw_ghost_cursor_cell(pos, num_hidden_board_rows, cell_size));
}

fn draw_ghost_cursor_cell(position: &Position, num_hidden_board_rows: usize, cell_size: f32) {
    let col_idx = position.x;
    let row_idx = position.y - num_hidden_board_rows as i32;

    let outline_color = BEIGE;
    draw_rectangle_lines(
        col_idx as f32 * cell_size + LINE_THICKNESS / 4.0,
        row_idx as f32 * cell_size + LINE_THICKNESS / 4.0,
        cell_size - LINE_THICKNESS / 2.0,
        cell_size - LINE_THICKNESS / 2.0,
        LINE_THICKNESS,
        outline_color,
    );
}

impl Renderer {
    pub fn draw_game_over_screen(&self, board: &Board) {
        clear_background(WHITE);

        let font_size = self.font_size;

        let y_base = screen_height() / 2.0;
        let final_score = board.score();
        let high_score = board.high_score();
        let lines = [
            String::from("Game Over"),
            format!("Final score: {final_score}"),
            format!("High score: {high_score}"),
            String::from("Press [enter] to play again, q to exit"),
        ];

        let opt_tallest_line = lines.iter().max_by_key(|line| {
            let dimensions = measure_text(line, None, font_size as _, 1.0);
            dimensions.height as i32
        });
        let size_of_tallest_line =
            measure_text(opt_tallest_line.unwrap(), None, font_size as _, 1.0);
        let line_spacing = size_of_tallest_line.height * 1.5;

        for (i, text) in lines.iter().enumerate() {
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                y_base + (i as f32 * line_spacing) + line_spacing / 2.,
                font_size,
                DARKGRAY,
            );
        }
    }
}
