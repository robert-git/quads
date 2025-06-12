#[derive(Clone)]
struct Cell {}

type Row = Vec<Cell>;

pub struct Board {
    rows: Vec<Row>,
}

use crate::tetromino_move::TetrominoMove;

impl Board {
    pub fn new() -> Self {
        let width = 5;
        let height = 10;
        Board {
            rows: vec![vec![Cell {}; width]; height],
        }
    }

    pub fn update(&mut self, tetromino_move: &TetrominoMove) {}
}
