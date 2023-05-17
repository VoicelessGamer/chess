use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData
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

  fn get_move_data(&self, origin: Position) -> MoveData {
    todo!()
  }
}
