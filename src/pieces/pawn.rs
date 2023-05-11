use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Pawn {
  row: usize,
  column: usize,
  white: bool
}

impl Pawn {
  pub fn new(row: usize, column: usize,white: bool) -> Pawn {
    Pawn { row, column, white }
  }
}

impl Piece for Pawn {
  fn get_position(&self) -> (usize, usize) {
    (self.row, self.column)
  }
  
  fn abbreviation(&self) -> String {
    return String::from("P");
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
    todo!()
  }
}
