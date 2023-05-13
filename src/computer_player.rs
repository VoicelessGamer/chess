use crate::{player::Player, position::Position};

pub struct ComputerPlayer {
  pub white: bool,
  pub difficulty: u32
}

impl Player for ComputerPlayer {
    fn get_move(&mut self) -> (Position, Position) {
        todo!()
    }
}