use crate::position::Position;

#[derive(Clone, Debug)]
pub struct MoveData {
  pub position: Position,                   // The current position of this piece
  pub attacks: Vec<Position>,               // Opposing pieces under attack by this piece
  pub defends: Vec<Position>,               // Friendly pieces defended by this piece
  pub pins: Vec<Position>,                  // Opposing pieces pinned to the king
  pub checking_path: Option<Vec<Position>>  // Path taken to attack the opposing king, if possible
}