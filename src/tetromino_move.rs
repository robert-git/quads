#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TetrominoMove {
    Down,
    Left,
    Right,
    RotateCW,
    RotateCCW,
}
