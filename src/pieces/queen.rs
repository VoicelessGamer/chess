use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Queen {
  row: usize,
  column: usize,
  white: bool
}

impl Queen {
  pub fn new(row: usize, column: usize,white: bool) -> Queen {
    Queen { row, column, white }
  }
}

impl Piece for Queen {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }
  
  fn abbreviation(&self) -> String {
    return String::from("Q");
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
