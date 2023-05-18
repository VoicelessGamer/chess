use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::chess_piece::ChessPiece
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

  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData {
    todo!()
  }
}
