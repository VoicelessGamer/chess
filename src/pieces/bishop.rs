use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::chess_piece::ChessPiece
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

  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData {
    todo!()
  }
}
