pub mod piece;

use super::position::Position;
use piece::Piece;

#[derive(Clone)]
pub struct Cursor {
    pub position: Position,
    pub piece: Piece,
}

impl Cursor {
    pub fn from(other: &Cursor, new_position: Position) -> Cursor {
        Cursor {
            position: new_position,
            piece: other.piece.clone(),
        }
    }
    pub fn get_point_positions(&self) -> Vec<Position> {
        Vec::new()
    }
}
