use crate::pieces::piece::Piece;
use crate::Board;

#[derive(Clone)]
pub struct Queen {
  white: bool,
}

impl Queen {
  pub fn new(white: bool) -> Queen {
    Queen { white }
  }
}

impl Piece for Queen {
  fn abbreviation(&self) -> String {
    return String::from("Q");
  }

  fn get_moves(&self, board: Board) -> Vec<(u8, u8)> {
    todo!()
  }
}
