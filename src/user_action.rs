use crate::tetromino_move::TetrominoMove;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserAction {
    SoftDrop,
    HardDrop,
    Left,
    Right,
    RotateCW,
    RotateCCW,
    Quit,
}

#[rustfmt::skip]
pub fn to_tetromino_move(action: UserAction) -> Option<TetrominoMove> {
    match action {
        UserAction::SoftDrop  => Some(TetrominoMove::UserSoftDown),
        UserAction::HardDrop  => Some(TetrominoMove::UserHardDown),
        UserAction::Left      => Some(TetrominoMove::Left),
        UserAction::Right     => Some(TetrominoMove::Right),
        UserAction::RotateCW  => Some(TetrominoMove::RotateCW),
        UserAction::RotateCCW => Some(TetrominoMove::RotateCCW),
        UserAction::Quit      => None,
    }
}
