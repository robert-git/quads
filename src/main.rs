mod board;
mod draw;
mod tetromino_move;
mod user_action;
mod user_move;

use board::Board;
use draw::Renderer;
use macroquad::color::colors::LIGHTGRAY;
use macroquad::prelude::{
    clear_background, get_keys_down, get_keys_pressed, is_key_pressed, next_frame,
    request_new_screen_size, screen_height, screen_width, KeyCode,
};
use std::time::{Duration, Instant};
use tetromino_move::TetrominoMove;
use user_action::to_tetromino_move;
use user_action::UserAction;
use user_move::UserMove;

const INPUT_DEBOUNCE: Duration = Duration::from_millis(50);
const BASELINE_CANVAS_WIDTH: f32 = 640.0;
const BASELINE_CANVAS_HEIGHT: f32 = 800.0;

#[macroquad::main("Quads")]
async fn main() {
    let canvas_size = get_window_dims(BASELINE_CANVAS_WIDTH, BASELINE_CANVAS_HEIGHT);
    let mut gp = initialize_game();

    let mut total_rows_cleared = 0;
    let mut next_row_thresh_for_speedup = 4;

    let mut renderer = Renderer::new(&canvas_size);

    while !gp.exit_game {
        if renderer.drawing_row_removal_animation() {
            clear_background(LIGHTGRAY);
            renderer.draw(&mut gp.board);
        } else if gp.game_over {
            renderer.draw_game_over_screen(&gp.board);
            reset_or_quit_game_when_apt(&mut gp);
        } else {
            request_new_screen_size(canvas_size.width, canvas_size.height);
            clear_background(LIGHTGRAY);

            let now = Instant::now();
            if now - gp.last_down_move_time > gp.auto_drop_interval {
                gp.opt_tetromino_move = Some(TetrominoMove::AutoDown);
                gp.last_down_move_time = now;
                println!("Auto down");
            } else if let Some(action) = get_user_action(now, &mut gp.last_key_time) {
                if action == UserAction::Quit {
                    gp.game_over = true;
                } else {
                    gp.opt_tetromino_move = to_tetromino_move(action);
                    if let Some(tetromino_move) = gp.opt_tetromino_move {
                        if tetromino_move == TetrominoMove::UM(UserMove::SoftDown) {
                            gp.last_down_move_time = now;
                        }
                        println!("tetromino_move {tetromino_move:?}");
                    }
                }
            }

            if let Some(tetromino_move) = gp.opt_tetromino_move {
                let (topped_out, num_rows_cleared_this_update) = gp.board.update(tetromino_move);
                if topped_out {
                    gp.game_over = true;
                }
                total_rows_cleared += num_rows_cleared_this_update;
                if total_rows_cleared >= next_row_thresh_for_speedup {
                    next_row_thresh_for_speedup += next_row_thresh_for_speedup;
                    gp.auto_drop_interval = scale_duration(gp.auto_drop_interval, 0.5);
                }
            }

            renderer.draw(&mut gp.board);

            gp.opt_tetromino_move = None;
        }

        next_frame().await;
    }
}

fn get_window_dims(requested_width: f32, requested_height: f32) -> draw::SizeInPixels {
    let aspect_ratio = requested_width / requested_height;
    let max_possible_width = screen_width().min(requested_width);
    let max_possible_height = screen_height().min(requested_height);
    let size_based_on_max_possible_height = draw::SizeInPixels {
        width: max_possible_height * aspect_ratio,
        height: max_possible_height,
    };
    let size_based_on_max_possible_width = draw::SizeInPixels {
        width: max_possible_width,
        height: max_possible_width / aspect_ratio,
    };

    if size_based_on_max_possible_height.height < size_based_on_max_possible_width.height {
        size_based_on_max_possible_height
    } else {
        size_based_on_max_possible_width
    }
}

struct GameParams {
    auto_drop_interval: Duration,
    last_down_move_time: Instant,
    opt_tetromino_move: Option<TetrominoMove>,
    last_key_time: Instant,
    board: Board,
    game_over: bool,
    exit_game: bool,
}

fn initialize_game() -> GameParams {
    let now = Instant::now();
    let auto_drop_interval = Duration::from_millis(2000);
    let last_down_move_time = now;
    let opt_tetromino_move = None;
    let last_key_time = now;
    let board = Board::new();
    let game_over = false;
    let exit_game = false;
    GameParams {
        auto_drop_interval,
        last_down_move_time,
        opt_tetromino_move,
        last_key_time,
        board,
        game_over,
        exit_game,
    }
}

fn reset_or_quit_game_when_apt(gp: &mut GameParams) {
    if is_key_pressed(KeyCode::Enter) {
        *gp = initialize_game();
    } else if is_key_pressed(KeyCode::Q) {
        gp.exit_game = true;
    }
}

fn scale_duration(duration: Duration, scale_factor: f64) -> Duration {
    let total_millis = duration.as_millis() as f64;
    let new_total_millis = total_millis * scale_factor;
    Duration::from_millis(new_total_millis.round() as u64)
}

fn get_user_action(now: Instant, last_key_time: &mut Instant) -> Option<UserAction> {
    if now - *last_key_time < INPUT_DEBOUNCE {
        return None;
    }

    {
        // Auto-repeat keys:
        let keys_down = get_keys_down();

        for key in keys_down {
            let opt_action = autorepeat_key_to_action(key);
            if opt_action.is_some() {
                *last_key_time = now;
                return opt_action;
            }
        }
    }

    {
        // Non-auto-repeat (single-shot) keys:
        let keys_pressed = get_keys_pressed();

        for key in keys_pressed {
            let opt_action = non_autorepeat_key_to_action(key);
            if let Some(action) = opt_action {
                println!("action = {action:?}");
                *last_key_time = now;
                return opt_action;
            }
        }
    }

    None
}

#[rustfmt::skip]
fn autorepeat_key_to_action(key: KeyCode) -> Option<UserAction> {
    match key {
        KeyCode::Down  => Some(UserAction::UM(UserMove::SoftDown)),
        KeyCode::Left  => Some(UserAction::UM(UserMove::Left)),
        KeyCode::Right => Some(UserAction::UM(UserMove::Right)),
        _              => None,
    }
}

#[rustfmt::skip]
fn non_autorepeat_key_to_action(key: KeyCode) -> Option<UserAction> {
    match key {
        KeyCode::Space => Some(UserAction::UM(UserMove::HardDown)),
        KeyCode::Up    => Some(UserAction::UM(UserMove::RotateCW)),
        KeyCode::Slash => Some(UserAction::UM(UserMove::RotateCCW)),
        KeyCode::Q     => Some(UserAction::Quit),
        _              => None,
    }
}
