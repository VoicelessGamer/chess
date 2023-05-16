use crate::position::Position;

pub trait Piece: Clone {
  fn is_white(&self) -> bool;
  fn get_moves(&self) -> Vec<Position>;
}