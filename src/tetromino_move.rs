#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TetrominoMove {
    AutoDown,
    UserDown,
    Left,
    Right,
    RotateCW,
    RotateCCW,
}
