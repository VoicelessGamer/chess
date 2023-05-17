use crate::position::Position;

#[derive(Clone, Debug)]
pub struct MoveData {
  pub origin: Position,
  pub attacks: Vec<Position>,
  pub defends: Vec<Position>,
  pub checking: bool
}