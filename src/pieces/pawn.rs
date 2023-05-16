use crate::{
  pieces::piece::Piece, 
  position::Position
};

#[derive(Clone)]
pub struct Pawn {
  white: bool
}

impl Pawn {
  pub fn new(white: bool) -> Pawn {
    Pawn { white }
  }
}

impl Piece for Pawn {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_moves(&self) -> Vec<Position> {
    todo!()
  }
}
