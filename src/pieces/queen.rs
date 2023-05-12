use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Queen {
  row: usize,
  column: usize,
  pub white: bool
}

impl Queen {
  pub fn new(row: usize, column: usize, white: bool) -> Queen {
    Queen { row, column, white }
  }
}

impl Piece for Queen {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
