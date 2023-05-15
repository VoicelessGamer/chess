use crate::{pieces::chess_piece::ChessPiece};

pub trait View {
  fn update_state(&mut self, _board: &Vec<Vec<Option<Box<ChessPiece>>>>);
}