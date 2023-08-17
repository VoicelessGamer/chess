use crate::{pieces::piece::Piece};

pub trait View {
  fn update_state(&mut self, board: &Vec<Vec<Option<Piece>>>);
}