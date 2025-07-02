pub use crate::user_move::UserMove;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TetrominoMove {
    AutoDown,
    UM(UserMove),
}

impl TetrominoMove {
    pub fn resets_down_timer(&self) -> bool {
        matches!(
            self,
            TetrominoMove::UM(UserMove::SoftDown) | TetrominoMove::UM(UserMove::HardDown)
        )
    }
}
