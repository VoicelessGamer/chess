use crate::{
  pieces::piece::Piece, 
  position::Position
};

#[derive(Clone)]
pub struct Rook {
  white: bool
}

impl Rook {
  pub fn new(white: bool) -> Rook {
    Rook { white }
  }
}

impl Piece for Rook {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_moves(&self) -> Vec<Position> {
    todo!()
  }
}
