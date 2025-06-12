#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TetrominoMove {
    AutoDown,
    UserSoftDown,
    UserHardDown,
    Left,
    Right,
    RotateCW,
    RotateCCW,
}
