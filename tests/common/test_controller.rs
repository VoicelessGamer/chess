use chess::{
  controller::Controller,
  piece_move::PieceMove
};

pub struct TestController {
  moves: Vec<PieceMove>,
  current: usize
}

impl TestController {
  pub fn new(moves: Vec<PieceMove>) -> Self {
    Self {
      moves,
      current: 0
    }
  }
}

impl Controller for TestController {
  fn get_move(&mut self, _white_turn: bool) -> PieceMove {
    let mv = self.moves[self.current].to_owned();
    self.current += 1;
    return mv;
  }
}