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
    fn get_rotation_origin_and_initial_point_positions(&self) -> (FloatPosition, Vec<Position>) {
        type RotationOrigin = FloatPosition;
        type P = Position;
        match self {
            Shape::O => {
                return (
                    RotationOrigin { x: 0.5, y: 0.5 },
                    vec![
                        P { x: 0, y: 0 },
                        P { x: 0, y: 1 },
                        P { x: 1, y: 0 },
                        P { x: 1, y: 1 },
                    ],
                )
            }
            Shape::I => {
                return (
                    RotationOrigin { x: -0.5, y: 0.5 },
                    vec![
                        P { x: -2, y: 0 },
                        P { x: -1, y: 0 },
                        P { x: 0, y: 0 },
                        P { x: 1, y: 0 },
                    ],
                )
            }
            Shape::T => {
                return (
                    RotationOrigin { x: 0.0, y: 1.0 },
                    vec![
                        P { x: 0, y: 1 },
                        P { x: -1, y: 1 },
                        P { x: 1, y: 1 },
                        P { x: 0, y: 0 },
                    ],
                )
            }
            Shape::S => {
                return (
                    RotationOrigin { x: 0.0, y: 1.0 },
                    vec![
                        P { x: -1, y: 1 },
                        P { x: 0, y: 1 },
                        P { x: 0, y: 0 },
                        P { x: 1, y: 0 },
                    ],
                )
            }
            Shape::Z => {
                return (
                    RotationOrigin { x: 0.0, y: 1.0 },
                    vec![
                        P { x: -1, y: 0 },
                        P { x: 0, y: 0 },
                        P { x: 0, y: 1 },
                        P { x: 1, y: 1 },
                    ],
                )
            }
            Shape::J => {
                return (
                    RotationOrigin { x: 0.0, y: 1.0 },
                    vec![
                        P { x: -1, y: 0 },
                        P { x: -1, y: 1 },
                        P { x: 0, y: 1 },
                        P { x: 1, y: 1 },
                    ],
                )
            }
            Shape::L => {
                return (
                    RotationOrigin { x: 0.0, y: 1.0 },
                    vec![
                        P { x: -1, y: 1 },
                        P { x: 0, y: 1 },
                        P { x: 1, y: 1 },
                        P { x: 1, y: 0 },
                    ],
                )
            }
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
    pub fn get_local_points(&self) -> &Vec<Position> {
        &self.local_point_positions
    }
}
