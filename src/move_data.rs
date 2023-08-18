use crate::position::Position;

#[derive(Clone, Debug)]
pub struct MoveData {
  pub position: Position,                   // The current position of this piece
  pub valid_moves: Vec<Position>,           // All positions this piece can move to (including opposing pieces under attack)
  pub attacks: Vec<Position>,               // TODO: This array will be a copy of valid_moves in most cases * All positions this piece has under attack
  pub defends: Vec<Position>,               // Friendly pieces defended by this piece
  pub pins: Vec<Position>,                  // Opposing pieces pinned to their king
  pub checking_path: Option<Vec<Position>>  // Path taken to attack the opposing king, if possible
}