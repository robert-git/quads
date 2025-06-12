mod action;
pub mod board;
mod tetromino_move;

use action::to_tetromino_move;
use action::Action;
use board::Board;
use macroquad::prelude::{KeyCode, get_keys_down, next_frame};
use tetromino_move::TetrominoMove;

use std::time::{Duration, Instant};

const DEBOUNCE: Duration = Duration::from_millis(50);

#[macroquad::main("Quads")]
async fn main() {
    let auto_drop_interval = Duration::from_millis(2000);
    let mut last_down_move_time = Instant::now();
    let mut run = true;
    let mut opt_tetromino_move = None;

    let mut last_key_time = Instant::now();
    let mut board = Board::new();

    while run {
        let opt_user_action = get_user_action(&mut last_key_time);

        let now = Instant::now();
        if now - last_down_move_time > auto_drop_interval {
            opt_tetromino_move = Some(TetrominoMove::Down);
            last_down_move_time = now;
            println!("Auto down");
        } else {
            if opt_user_action.is_some() {
                let action = opt_user_action.unwrap();
                // println!("action {:?}", action);
                if action == Action::Quit {
                    run = false;
                } else {
                    opt_tetromino_move = to_tetromino_move(action);
                    if opt_tetromino_move.is_some() {
                        let tetromino_move = opt_tetromino_move.unwrap();
                        if tetromino_move == TetrominoMove::Down {
                            last_down_move_time = now;
                        }
                        println!("tetromino_move {:?}", tetromino_move);
                    }
                }
            }
        }
        if opt_tetromino_move.is_some() {
            board.update(opt_tetromino_move.unwrap());
        }
        board.draw();
        next_frame().await;
        opt_tetromino_move = None;
    }
}

fn get_user_action(last_key_time: &mut Instant) -> Option<Action> {
    let now = Instant::now();
    if now - *last_key_time < DEBOUNCE {
        return None;
    }

    let keys_down = get_keys_down();

    for key in keys_down {
        let opt_action = to_action(key);
        if opt_action.is_some() {
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
