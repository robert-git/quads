#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    Down,
    Left,
    Right,
    RotateCW,
    RotateCCW,
    Quit,
}
