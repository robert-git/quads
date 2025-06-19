use super::Position;

#[derive(Copy, Clone)]
pub enum Shape {
    O,
    I,
    T,
    S,
    Z,
    J,
    L,
}

impl Shape {
    fn get_rotation_origin_and_initial_point_positions(self) -> (FloatPosition, Vec<Position>) {
        type RotationOrigin = FloatPosition;
        type P = Position;
        match self {
            Shape::O => (
                RotationOrigin { x: 0.5, y: 0.5 },
                vec![
                    P { x: 0, y: 0 },
                    P { x: 0, y: 1 },
                    P { x: 1, y: 0 },
                    P { x: 1, y: 1 },
                ],
            ),
            Shape::I => (
                RotationOrigin { x: -0.5, y: 0.5 },
                vec![
                    P { x: -2, y: 0 },
                    P { x: -1, y: 0 },
                    P { x: 0, y: 0 },
                    P { x: 1, y: 0 },
                ],
            ),
            Shape::T => (
                RotationOrigin { x: 0.0, y: 1.0 },
                vec![
                    P { x: 0, y: 1 },
                    P { x: -1, y: 1 },
                    P { x: 1, y: 1 },
                    P { x: 0, y: 0 },
                ],
            ),
            Shape::S => (
                RotationOrigin { x: 0.0, y: 1.0 },
                vec![
                    P { x: -1, y: 1 },
                    P { x: 0, y: 1 },
                    P { x: 0, y: 0 },
                    P { x: 1, y: 0 },
                ],
            ),
            Shape::Z => (
                RotationOrigin { x: 0.0, y: 1.0 },
                vec![
                    P { x: -1, y: 0 },
                    P { x: 0, y: 0 },
                    P { x: 0, y: 1 },
                    P { x: 1, y: 1 },
                ],
            ),
            Shape::J => (
                RotationOrigin { x: 0.0, y: 1.0 },
                vec![
                    P { x: -1, y: 0 },
                    P { x: -1, y: 1 },
                    P { x: 0, y: 1 },
                    P { x: 1, y: 1 },
                ],
            ),
            Shape::L => (
                RotationOrigin { x: 0.0, y: 1.0 },
                vec![
                    P { x: -1, y: 1 },
                    P { x: 0, y: 1 },
                    P { x: 1, y: 1 },
                    P { x: 1, y: 0 },
                ],
            ),
        }
    }
}

#[derive(Copy, Clone)]
struct FloatPosition {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone)]
pub struct Piece {
    shape: Shape,
    local_point_positions: Vec<Position>,
    local_rotation_origin: FloatPosition,
}

impl Piece {
    pub fn new(shape: Shape) -> Self {
        let (local_rotation_origin, local_point_positions) =
            shape.get_rotation_origin_and_initial_point_positions();
        Piece {
            shape,
            local_point_positions,
            local_rotation_origin,
        }
    }

    pub fn from(&self, local_point_positions: Vec<Position>) -> Self {
        Piece {
            shape: self.shape,
            local_point_positions,
            local_rotation_origin: self.local_rotation_origin,
        }
    }

    pub fn rotate_cw_copy(&self) -> Self {
        rotate_90_deg(self, &RotationDir::Clockwise)
    }

    pub fn rotate_ccw_copy(&self) -> Self {
        rotate_90_deg(self, &RotationDir::Counterclockwise)
    }

    pub fn get_local_points(&self) -> &Vec<Position> {
        &self.local_point_positions
    }
}

enum RotationDir {
    Counterclockwise,
    Clockwise,
}

fn rotate_90_deg(piece: &Piece, dir: &RotationDir) -> Piece {
    let mut float_points_centered_at_origin =
        offset_to_center(&piece.local_point_positions, &piece.local_rotation_origin);

    swap_xs_and_ys(&mut float_points_centered_at_origin);

    // The coordinate origin of the board is the upper left corner, such that
    // positive x is to the right and positive y is down.
    // Therefore after swapping x and y, to rotate clockwise, negate the xs.
    // If the coordinate plane were instead a normal Cartesian plane where positive y is
    // upwards, then clockwise rotation would instead require negating the ys.
    match dir {
        RotationDir::Clockwise        => negate_xs(&mut float_points_centered_at_origin),
        RotationDir::Counterclockwise => negate_ys(&mut float_points_centered_at_origin),
    }

    let local_point_positions = offset_from_center(
        &float_points_centered_at_origin,
        &piece.local_rotation_origin,
    );

    piece.from(local_point_positions)
}

fn offset_to_center(positions: &[Position], center: &FloatPosition) -> Vec<FloatPosition> {
    return positions
        .iter()
        .map(|&pos| FloatPosition {
            x: pos.x as f64 - center.x,
            y: pos.y as f64 - center.y,
        })
        .collect();
}

fn swap_xs_and_ys(float_posns: &mut [FloatPosition]) {
    float_posns
        .iter_mut()
        .for_each(|pos| std::mem::swap(&mut pos.x, &mut pos.y));
}

fn negate_xs(float_posns: &mut [FloatPosition]) {
    float_posns.iter_mut().for_each(|pos| pos.x = -pos.x);
}

fn negate_ys(float_posns: &mut [FloatPosition]) {
    float_posns.iter_mut().for_each(|pos| pos.y = -pos.y);
}

fn offset_from_center(
    float_positions: &[FloatPosition],
    center: &FloatPosition,
) -> Vec<Position> {
    return float_positions
        .iter()
        .map(|&float_pos| Position {
            x: round_to_nearest_half(float_pos.x + center.x) as _,
            y: round_to_nearest_half(float_pos.y + center.y) as _,
        })
        .collect();
}

fn round_to_nearest_half(x: f64) -> f64 {
    (x * 2.0).round() / 2.0
}
