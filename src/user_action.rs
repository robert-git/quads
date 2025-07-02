use crate::tetromino_move::TetrominoMove;
use crate::user_move::UserMove;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserAction {
    UM(UserMove),
    Quit,
}

#[rustfmt::skip]
pub fn to_tetromino_move(action: UserAction) -> Option<TetrominoMove> {
    match action {
        UserAction::UM(user_move) => Some(TetrominoMove::UM(user_move)),
        UserAction::Quit          => None,
    }
}
