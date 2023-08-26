use crate::{pieces::piece::Piece, game::State};

pub trait View {
  /**
   * This function should update the view of both players with the supplied game state.
   */
  fn update_state(&mut self, board: &Vec<Vec<Option<Piece>>>, game_state: State);
}