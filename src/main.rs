mod action;
pub mod board;
mod tetromino_move;

use action::to_tetromino_move;
use action::Action;
use board::Board;
use macroquad::prelude::*;
use tetromino_move::TetrominoMove;

use std::collections::LinkedList;
use std::time::{Duration, Instant};

const SQUARES: i16 = 16;
const DEBOUNCE: Duration = Duration::from_millis(250);

type Point = (i16, i16);

struct Snake {
    head: Point,
    body: LinkedList<Point>,
    dir: Point,
}

#[macroquad::main("Quads")]
async fn main() {
    let auto_drop_interval = Duration::from_millis(2000);
    let mut last_down_move_time = Instant::now();
    let mut run = true;
    let mut tetromino_move = TetrominoMove::Down;

    let mut last_key_time = Instant::now();
    let mut board = Board::new();

    while run {
        let opt_user_action = get_user_action(&mut last_key_time);

        let now = Instant::now();
        if opt_user_action.is_some() {
            let action = opt_user_action.unwrap();
            // println!("action {:?}", action);
            if action == Action::Quit {
                run = false;
            } else {
                let opt_tetromino_move = to_tetromino_move(action);
                if opt_tetromino_move.is_some() {
                    tetromino_move = opt_tetromino_move.unwrap();
                    if tetromino_move == TetrominoMove::Down {
                        last_down_move_time = now;
                    }
                    println!("tetromino_move {:?}", tetromino_move);
                }
            }
        }

        if now - last_down_move_time > auto_drop_interval {
            last_down_move_time = now;
            println!("Auto down");
        }

        board.update(&tetromino_move);
        board.draw();
        next_frame().await;
    }
    #[cfg(feature = "")]
    {
        let mut snake = Snake {
            head: (0, 0),
            dir: (1, 0),
            body: LinkedList::new(),
        };
        let mut fruit: Point = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
        let mut score = 0;
        let mut interval = 0.3;
        let mut last_update = get_time();
        let mut navigation_lock = false;
        let mut game_over = false;

        loop {
            if !game_over {
                handle_keypress(&mut snake.dir, &mut navigation_lock);

                if get_time() - last_update > interval {
                    last_update = get_time();
                    snake.body.push_front(snake.head);
                    snake.head = (snake.head.0 + snake.dir.0, snake.head.1 + snake.dir.1);
                    if snake.head == fruit {
                        fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                        score += 100;
                        interval *= 0.9;
                    } else {
                        snake.body.pop_back();
                    }
                    if snake.head.0 < 0
                        || snake.head.1 < 0
                        || snake.head.0 >= SQUARES
                        || snake.head.1 >= SQUARES
                    {
                        game_over = true;
                    }
                    for (x, y) in &snake.body {
                        if *x == snake.head.0 && *y == snake.head.1 {
                            game_over = true;
                        }
                    }
                    navigation_lock = false;
                }
            }
            if !game_over {
                clear_background(LIGHTGRAY);

                let game_size = screen_width().min(screen_height());
                let offset_x = (screen_width() - game_size) / 2. + 10.;
                let offset_y = (screen_height() - game_size) / 2. + 10.;
                let sq_size = (screen_height() - offset_y * 2.) / SQUARES as f32;

                draw_rectangle(offset_x, offset_y, game_size - 20., game_size - 20., WHITE);

                // draw horizontal grid lines
                for i in 1..SQUARES {
                    draw_line(
                        offset_x,
                        offset_y + sq_size * i as f32,
                        screen_width() - offset_x,
                        offset_y + sq_size * i as f32,
                        2.,
                        LIGHTGRAY,
                    );
                }

                // draw vertical grid lines
                for i in 1..SQUARES {
                    draw_line(
                        offset_x + sq_size * i as f32,
                        offset_y,
                        offset_x + sq_size * i as f32,
                        screen_height() - offset_y,
                        2.,
                        LIGHTGRAY,
                    );
                }

                draw_rectangle(
                    offset_x + snake.head.0 as f32 * sq_size,
                    offset_y + snake.head.1 as f32 * sq_size,
                    sq_size,
                    sq_size,
                    DARKGREEN,
                );

                for (x, y) in &snake.body {
                    draw_rectangle(
                        offset_x + *x as f32 * sq_size,
                        offset_y + *y as f32 * sq_size,
                        sq_size,
                        sq_size,
                        LIME,
                    );
                }

                draw_rectangle(
                    offset_x + fruit.0 as f32 * sq_size,
                    offset_y + fruit.1 as f32 * sq_size,
                    sq_size,
                    sq_size,
                    GOLD,
                );

                draw_text(format!("SCORE: {score}").as_str(), 10., 20., 20., DARKGRAY);
            } else {
                clear_background(WHITE);
                let text = "Game Over. Press [enter] to play again.";
                let font_size = 30.;
                let text_size = measure_text(text, None, font_size as _, 1.0);

                draw_text(
                    text,
                    screen_width() / 2. - text_size.width / 2.,
                    screen_height() / 2. + text_size.height / 2.,
                    font_size,
                    DARKGRAY,
                );

                if is_key_down(KeyCode::Enter) {
                    snake = Snake {
                        head: (0, 0),
                        dir: (1, 0),
                        body: LinkedList::new(),
                    };
                    fruit = (rand::gen_range(0, SQUARES), rand::gen_range(0, SQUARES));
                    score = 0;
                    interval = 0.3;
                    last_update = get_time();
                    game_over = false;
                }
            }
            next_frame().await;
        }
    }
}

fn handle_keypress(dir: &mut Point, navigation_lock: &mut bool) {
    const UP: Point = (0, -1);
    const DOWN: Point = (0, 1);
    const RIGHT: Point = (1, 0);
    const LEFT: Point = (-1, 0);

    if is_key_down(KeyCode::Right) && *dir != LEFT && !*navigation_lock {
        *dir = RIGHT;
        *navigation_lock = true;
    } else if is_key_down(KeyCode::Left) && *dir != RIGHT && !*navigation_lock {
        *dir = LEFT;
        *navigation_lock = true;
    } else if is_key_down(KeyCode::Up) && *dir != DOWN && !*navigation_lock {
        *dir = UP;
        *navigation_lock = true;
    } else if is_key_down(KeyCode::Down) && *dir != UP && !*navigation_lock {
        *dir = DOWN;
        *navigation_lock = true;
    }
}

fn get_key_press_or_timeout() -> Option<char> {
    get_char_pressed()
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
