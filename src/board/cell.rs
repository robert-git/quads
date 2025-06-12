#[derive(PartialEq, Copy, Clone)]
pub enum State {
    Empty,
    Cursor,
    Stack,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub state: State,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            state: State::Empty,
        }
    }
}
