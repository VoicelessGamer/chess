use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Rook {
  row: usize,
  column: usize,
  white: bool
}

impl Rook {
  pub fn new(row: usize, column: usize,white: bool) -> Rook {
    Rook { row, column, white }
  }
}

impl Piece for Rook {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }
  
  fn abbreviation(&self) -> String {
    return String::from("R");
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
