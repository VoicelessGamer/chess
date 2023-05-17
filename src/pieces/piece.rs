use crate::{
  move_data::MoveData, 
  position::Position
};

pub trait Piece: Clone {
  fn is_white(&self) -> bool;
  fn get_move_data(&self, origin: Position) -> MoveData;
}