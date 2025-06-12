mod action;
mod board;
mod draw;
mod tetromino_move;

use action::to_tetromino_move;
use action::Action;
use board::Board;
use macroquad::color::colors::LIGHTGRAY;
use macroquad::prelude::{
    clear_background, get_keys_down, is_key_down, next_frame, request_new_screen_size, KeyCode,
};
use std::time::{Duration, Instant};
use tetromino_move::TetrominoMove;

const INPUT_DEBOUNCE: Duration = Duration::from_millis(50);
const ROTATION_DEBOUNCE: Duration = Duration::from_millis(150);
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
                gp.opt_tetromino_move = Some(TetrominoMove::Down);
                gp.last_down_move_time = now;
                println!("Auto down");
            } else {
                if opt_user_action.is_some() {
                    let action = opt_user_action.unwrap();
                    if action == Action::Quit {
                        gp.game_over = true;
                    } else {
                        gp.opt_tetromino_move = to_tetromino_move(action);
                        if gp.opt_tetromino_move.is_some() {
                            let tetromino_move = gp.opt_tetromino_move.unwrap();
                            if tetromino_move == TetrominoMove::Down {
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

fn get_user_action(last_key_time: &mut Instant) -> Option<Action> {
    let now = Instant::now();
    if now - *last_key_time < INPUT_DEBOUNCE {
        return None;
    }

    let keys_down = get_keys_down();

    for key in keys_down {
        let opt_action = to_action(key);
        if opt_action.is_some() {
            let action = opt_action.unwrap();
            if action == Action::RotateCW || action == Action::RotateCCW {
                if now - *last_key_time < ROTATION_DEBOUNCE {
                    return None;
                }
            }
            *last_key_time = now;
        }
        return opt_action;
    }

    None
}

#[rustfmt::skip]
fn to_action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Down  => Some(Action::Down),
        KeyCode::Left  => Some(Action::Left),
        KeyCode::Right => Some(Action::Right),
        KeyCode::Up    => Some(Action::RotateCW),
        KeyCode::Slash => Some(Action::RotateCCW),
        KeyCode::Q     => Some(Action::Quit),
        _              => None,
    }
}
