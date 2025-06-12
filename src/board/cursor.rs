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
        let local_points = self.piece.get_local_points().clone();
        return offset_points_by_position(local_points, &self.position);
    }
}

fn offset_points_by_position(mut points: Vec<Position>, pos: &Position) -> Vec<Position> {
    points.iter_mut().for_each(|point| {
        point.x += pos.x;
        point.y += pos.y;
    });
    points
}
