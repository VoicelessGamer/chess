use crate::{
  pieces::piece::Piece, 
  position::Position
};


#[derive(Clone)]
pub struct Bishop {
  white: bool
}

impl Bishop {
  pub fn new(white: bool) -> Bishop {
    Bishop { white }
  }
}

impl Piece for Bishop {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_moves(&self) -> Vec<Position> {
    todo!()
  }
}
