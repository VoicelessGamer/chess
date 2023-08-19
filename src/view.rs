use crate::{pieces::piece::Piece, game::State};

pub trait View {
  fn update_state(&mut self, board: &Vec<Vec<Option<Piece>>>, game_state: State);
}