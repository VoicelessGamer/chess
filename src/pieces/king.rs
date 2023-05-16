use crate::{
  pieces::piece::Piece, 
  position::Position
};


#[derive(Clone)]
pub struct King {
  white: bool
}

impl King {
  pub fn new(white: bool) -> King {
    King { white }
  }
}

impl Piece for King {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_moves(&self) -> Vec<Position> {
    todo!()
  }
}
