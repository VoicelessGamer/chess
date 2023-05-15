use crate::{position::Position};

pub trait Controller {
  fn get_white_move(&self) -> (Position, Position);
  fn get_black_move(&self) -> (Position, Position);
}