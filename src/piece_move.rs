use crate::position::Position;

#[derive(Clone)]
pub struct PieceMove {
  pub current: Position,
  pub target: Position
}