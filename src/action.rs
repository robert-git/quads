use crate::tetromino_move::TetrominoMove;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    Down,
    Left,
    Right,
    RotateCW,
    RotateCCW,
    Quit,
}

pub fn to_tetromino_move(action: Action) -> Option<TetrominoMove> {
    match action {
        Action::Down      => Some(TetrominoMove::Down),
        Action::Left      => Some(TetrominoMove::Left),
        Action::Right     => Some(TetrominoMove::Right),
        Action::RotateCW  => Some(TetrominoMove::RotateCW),
        Action::RotateCCW => Some(TetrominoMove::RotateCCW),
        _                 => None,
    }
}
