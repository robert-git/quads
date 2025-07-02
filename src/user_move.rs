#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserMove {
    SoftDown,
    HardDown,
    Left,
    Right,
    RotateCW,
    RotateCCW,
}
