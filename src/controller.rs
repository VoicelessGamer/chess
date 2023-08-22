use crate::piece_move::PieceMove;

pub trait Controller {
  fn get_move(&self, white_turn: bool) -> PieceMove;
}