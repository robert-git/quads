mod board;
mod draw;
mod tetromino_move;
mod user_action;

use board::Board;
use macroquad::color::colors::LIGHTGRAY;
use macroquad::prelude::{
    clear_background, get_keys_down, get_keys_pressed, is_key_down, next_frame,
    request_new_screen_size, KeyCode,
};
use std::time::{Duration, Instant};
use tetromino_move::TetrominoMove;
use user_action::to_tetromino_move;
use user_action::UserAction;

const INPUT_DEBOUNCE: Duration = Duration::from_millis(50);
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 800.0;
const RIGHT_MARGIN_WIDTH: f32 = 80.0;
const BOTTOM_MARGIN_HEIGHT: f32 = 20.0;
const BOARD_WIDTH: f32 = WINDOW_WIDTH - RIGHT_MARGIN_WIDTH;
const BOARD_HEIGHT: f32 = WINDOW_HEIGHT - BOTTOM_MARGIN_HEIGHT;

#[macroquad::main("Quads")]
async fn main() {
    let mut gp = initialize_game();

    loop {
        if gp.game_over {
            draw::draw_game_over_screen(&gp.board);
            reset_game_when_apt(&mut gp);
        } else {
            request_new_screen_size(WINDOW_WIDTH, WINDOW_HEIGHT);
            clear_background(LIGHTGRAY);

            let opt_user_action = get_user_action(&mut gp.last_key_time);

            let now = Instant::now();
            if now - gp.last_down_move_time > gp.auto_drop_interval {
                gp.opt_tetromino_move = Some(TetrominoMove::AutoDown);
                gp.last_down_move_time = now;
                println!("Auto down");
            } else {
                if opt_user_action.is_some() {
                    let action = opt_user_action.unwrap();
                    if action == UserAction::Quit {
                        gp.game_over = true;
                    } else {
                        gp.opt_tetromino_move = to_tetromino_move(action);
                        if gp.opt_tetromino_move.is_some() {
                            let tetromino_move = gp.opt_tetromino_move.unwrap();
                            if tetromino_move == TetrominoMove::UserSoftDown {
                                gp.last_down_move_time = now;
                            }
                            println!("tetromino_move {:?}", tetromino_move);
                        }
                    }
                }
            }

            if gp.opt_tetromino_move.is_some() {
                let topped_out = gp.board.update(gp.opt_tetromino_move.unwrap());
                if topped_out {
                    gp.game_over = true;
                }
            }

            draw::draw(
                &gp.board,
                draw::SizeInPixels {
                    width: BOARD_WIDTH,
                    height: BOARD_HEIGHT,
                },
            );

            gp.opt_tetromino_move = None;
        }

        next_frame().await;
    }
}

struct GameParams {
    auto_drop_interval: Duration,
    last_down_move_time: Instant,
    opt_tetromino_move: Option<TetrominoMove>,
    last_key_time: Instant,
    board: Board,
    game_over: bool,
}

fn initialize_game() -> GameParams {
    let auto_drop_interval = Duration::from_millis(2000);
    let last_down_move_time = Instant::now();
    let opt_tetromino_move = None;
    let last_key_time = Instant::now();
    let board = Board::new();
    let game_over = false;
    GameParams {
        auto_drop_interval,
        last_down_move_time,
        opt_tetromino_move,
        last_key_time,
        board,
        game_over,
    }
}

fn reset_game_when_apt(gp: &mut GameParams) {
    if is_key_down(KeyCode::Enter) {
        *gp = initialize_game();
    }
}

fn get_user_action(last_key_time: &mut Instant) -> Option<UserAction> {
    let now = Instant::now();
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
            if opt_action.is_some() {
                println!("action = {:?}", opt_action.unwrap());
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
        KeyCode::Down  => Some(UserAction::SoftDrop),
        KeyCode::Left  => Some(UserAction::Left),
        KeyCode::Right => Some(UserAction::Right),
        _              => None,
    }
}

#[rustfmt::skip]
fn non_autorepeat_key_to_action(key: KeyCode) -> Option<UserAction> {
    match key {
        KeyCode::Space => Some(UserAction::HardDrop),
        KeyCode::Up    => Some(UserAction::RotateCW),
        KeyCode::Slash => Some(UserAction::RotateCCW),
        KeyCode::Q     => Some(UserAction::Quit),
        _              => None,
    }
}
