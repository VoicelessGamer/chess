use crate::{
  pieces::piece::Piece, 
  position::Position
};

#[derive(Clone)]
pub struct Queen {
  white: bool
}

impl Queen {
  pub fn new(white: bool) -> Queen {
    Queen { white }
  }
}

impl Piece for Queen {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_moves(&self) -> Vec<Position> {
    todo!()
  }
}
