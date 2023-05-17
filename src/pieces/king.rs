use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData
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

  fn get_move_data(&self, origin: Position) -> MoveData {
    todo!()
  }
}
