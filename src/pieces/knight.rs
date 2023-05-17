use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData
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

  fn get_move_data(&self, origin: Position) -> MoveData {
    todo!()
  }
}
