use crate::pieces::piece::Piece;
use crate::Board;

#[derive(Clone)]
pub struct Rook {
  white: bool,
}

impl Rook {
  pub fn new(white: bool) -> Rook {
    Rook { white }
  }
}

impl Piece for Rook {
  fn abbreviation(&self) -> String {
    return String::from("R");
  }

  fn get_moves(&self, board: Board) -> Vec<(u8, u8)> {
    todo!()
  }
}
