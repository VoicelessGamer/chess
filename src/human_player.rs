use crate::{player::Player, position::Position};

pub struct HumanPlayer {
  pub white: bool
}

impl Player for HumanPlayer {
    fn get_move(&mut self) -> (Position, Position) {
        //todo!()
        return (Position {row: 1, column: 3}, Position {row: 3, column: 3})
    }
}