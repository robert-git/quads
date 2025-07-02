use crate::user_move::UserMove;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UserAction {
    UM(UserMove),
    Quit,
}
