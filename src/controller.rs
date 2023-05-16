use crate::{player_move::PlayerMove};

pub trait Controller {
  fn get_move(&self, white_turn: bool) -> PlayerMove;
}