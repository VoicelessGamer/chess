use crate::Board;

pub trait Piece {
  fn abbreviation(&self) -> String;
  fn get_moves(&self, board:Board) -> Vec<(u8, u8)>;
}