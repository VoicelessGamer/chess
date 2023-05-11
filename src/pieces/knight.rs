use crate::pieces::piece::Piece;
use crate::Board;

#[derive(Clone)]
pub struct Knight {
  white: bool,
}

impl Knight {
  pub fn new(white: bool) -> Knight {
    Knight { white }
  }
}

impl Piece for Knight {
  fn abbreviation(&self) -> String {
    return String::from("N");
  }

  fn get_moves(&self, board: Board) -> Vec<(u8, u8)> {
    todo!()
  }
}
