use crate::position::Position;

pub trait Player {
  fn get_move(&mut self) -> (Position, Position); // (Current position, Selected position)
}