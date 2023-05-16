use std::io;

use crate::{
  controller::Controller,
  position::Position, 
  player_move::PlayerMove
};

pub struct IOController {
  white_human: bool,
  black_human: bool
  //ai_engine: A
}

impl IOController {
  pub fn new(white_human: bool, black_human: bool) -> Self {
    Self {
      white_human,
      black_human
    }
  }

  /**
   * Retrieves the next move for white 
   */
  fn get_white_move(&self) -> PlayerMove {
    if self.white_human {
      return get_move_input();
    } else {
      return get_move_input(); // TODO: Change later for ai implementation
    }
  }

  /**
   * Retrieves the next move for black 
   */
  fn get_black_move(&self) -> PlayerMove {
    if self.black_human {
      return get_move_input();
    } else {
      return get_move_input(); // TODO: Change later for ai implementation
    }
  }
}

impl Controller for IOController {
  /**
   * Retrieves the next chosen move from the white or black player based on
   * the provided white_turn bool parameter
   */
  fn get_move(&self, white_turn: bool) -> PlayerMove {
    if white_turn {
      return self.get_white_move();
    } else {
      return self.get_black_move();
    }
  }
}

/**
 * A function to retrieve the input of the next move from the terminal input.
 * Loops on the input string until it is valid
 */
fn get_move_input() -> PlayerMove {
  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_goes_into_input_above) => {},
      Err(_no_updates_is_fine) => {},
    }
    input = input.trim().to_string();

    let split: Vec<&str> = input.split(",").collect();
    if split.len() == 4 {
      return PlayerMove {
        current: Position {
          row: split[0].parse::<usize>().unwrap(), 
          column: split[1].parse::<usize>().unwrap()
        }, 
        target: Position {
          row: split[2].parse::<usize>().unwrap(), 
          column: split[3].parse::<usize>().unwrap()
        }
      }
    }
    println!("Invalid Input");
  }
}