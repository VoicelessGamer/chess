use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Rook {
  row: usize,
  column: usize,
  pub white: bool
}

impl Rook {
  pub fn new(row: usize, column: usize, white: bool) -> Rook {
    Rook { row, column, white }
  }
}

impl Piece for Rook {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
