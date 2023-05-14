use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Bishop {
  row: usize,
  column: usize,
  pub white: bool
}

impl Bishop {
  pub fn new(row: usize, column: usize, white: bool) -> Bishop {
    Bishop { row, column, white }
  }
}

impl Piece for Bishop {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
