use crate::tetromino_move::TetrominoMove;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserAction {
    Down,
    Left,
    Right,
    RotateCW,
    RotateCCW,
    Quit,
}

#[rustfmt::skip]
pub fn to_tetromino_move(action: UserAction) -> Option<TetrominoMove> {
    match action {
        UserAction::Down      => Some(TetrominoMove::UserDown),
        UserAction::Left      => Some(TetrominoMove::Left),
        UserAction::Right     => Some(TetrominoMove::Right),
        UserAction::RotateCW  => Some(TetrominoMove::RotateCW),
        UserAction::RotateCCW => Some(TetrominoMove::RotateCCW),
        _                     => None,
    }
}
