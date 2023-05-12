use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct King {
  row: usize,
  column: usize,
  pub white: bool
}

impl King {
  pub fn new(row: usize, column: usize, white: bool) -> King {
    King { row, column, white }
  }
}

impl Piece for King {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
