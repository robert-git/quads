use crate::piece_move::PieceMove;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Action {
    Down,
    Left,
    Right,
    RotateCW,
    RotateCCW,
    Quit,
}

pub fn to_piece_move(action: Action) -> Option<PieceMove> {
    match action {
        Action::Down      => Some(PieceMove::Down),
        Action::Left      => Some(PieceMove::Left),
        Action::Right     => Some(PieceMove::Right),
        Action::RotateCW  => Some(PieceMove::RotateCW),
        Action::RotateCCW => Some(PieceMove::RotateCCW),
        _                 => None,
    }
}
