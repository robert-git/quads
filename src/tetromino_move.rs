pub use crate::user_move::UserMove;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TetrominoMove {
    AutoDown,
    UM(UserMove),
}
