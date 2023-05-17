use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData
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

  fn get_move_data(&self, origin: Position) -> MoveData {
    todo!()
  }
}
