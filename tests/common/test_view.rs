use chess::{
  view::View,
  pieces::piece::Piece, game::State
};

pub struct TestView {}

impl View for TestView {
  fn update_state(&mut self, _board: &Vec<Vec<Option<Piece>>>, _game_state: State) {}
}