use crate::model::Position;

#[derive(Clone, Debug)]
pub struct MoveData {
  /// The current position of this piece
  pub position: Position,
  /// All positions this piece can move to (including opposing pieces under attack)
  pub valid_moves: Vec<Position>,
  /// All positions this piece has under attack
  pub attacks: Vec<Position>,
  /// Friendly pieces defended by this piece
  pub defends: Vec<Position>,
  /// Opposing pieces pinned to their king
  pub pins: Vec<Position>,
  /// Path taken to attack the opposing king, if possible
  pub checking_path: Option<Vec<Position>>
}