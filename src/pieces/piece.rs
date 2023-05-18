use crate::{
  move_data::MoveData, 
  position::Position,
  pieces::chess_piece::ChessPiece
};

pub trait Piece: Clone {
  fn is_white(&self) -> bool;
  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData;
}