use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Knight {
  row: usize,
  column: usize,
  pub white: bool
}

impl Knight {
  pub fn new(row: usize, column: usize, white: bool) -> Knight {
    Knight { row, column, white }
  }
}

impl Piece for Knight {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
