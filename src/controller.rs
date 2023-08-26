use crate::piece_move::PieceMove;

pub trait Controller {
  /**
   * This function should retrieve the next move for the active player
   */
  fn get_move(&mut self, white_turn: bool) -> PieceMove;
}