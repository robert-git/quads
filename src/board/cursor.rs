pub mod piece;

use super::position::Position;
use macroquad::prelude::rand;
use piece::Piece;
use piece::Shape;

#[derive(Clone)]
pub struct Cursor {
    pub position: Position,
    pub piece: Piece,
}

impl Cursor {
    pub fn from_random_shape_in_list(shape_list: &[Shape], position: Position) -> Cursor {
        Cursor {
            position,
            piece: Piece::new(random_shape(shape_list)),
        }
    }

    pub fn offset_copy(&self, new_position: Position) -> Cursor {
        Cursor {
            position: new_position,
            piece: self.piece.clone(),
        }
    }

    pub fn rotate_cw_copy(&self) -> Cursor {
        Cursor {
            position: self.position,
            piece: self.piece.rotate_cw_copy(),
        }
    }

    pub fn rotate_ccw_copy(&self) -> Cursor {
        Cursor {
            position: self.position,
            piece: self.piece.rotate_ccw_copy(),
        }
    }

    pub fn get_point_positions(&self) -> Vec<Position> {
        let local_points = self.piece.get_local_points().clone();
        offset_points_by_position(local_points, self.position)
    }
}

fn random_shape(shape_list: &[Shape]) -> Shape {
    shape_list[rand::gen_range(0, shape_list.len())]
}

fn offset_points_by_position(mut points: Vec<Position>, pos: Position) -> Vec<Position> {
    for point in &mut points {
        point.x += pos.x;
        point.y += pos.y;
    }
    points
}
