use crate::{
  pieces::piece::Piece, 
  position::Position
};


#[derive(Clone)]
pub struct Knight {
  white: bool
}

impl Knight {
  pub fn new(white: bool) -> Knight {
    Knight { white }
  }
}

impl Piece for Knight {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_moves(&self) -> Vec<Position> {
    todo!()
  }
}
