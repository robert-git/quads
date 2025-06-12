#[derive(Copy, Clone)]
pub enum State {
    Empty,
    Cursor,
    Stack,
}

#[derive(Clone)]
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
