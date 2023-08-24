use crate::position::Position;

#[derive(Clone)]
pub struct PieceMove {
  pub start: Position,
  pub end: Position,
  pub promotion: Option<String>
}