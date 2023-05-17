use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData
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

  fn get_move_data(&self, origin: Position) -> MoveData {
    todo!()
  }
}
