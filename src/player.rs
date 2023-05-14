use crate::{
  position::Position, 
  pieces::chess_piece::ChessPiece
};

pub trait Player {
  fn update_state(&mut self, _board: &Vec<Vec<Option<Box<ChessPiece>>>>) {}
  fn get_move(&mut self) -> (Position, Position); // (Current position, Selected position)
}