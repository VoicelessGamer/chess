use std::io;

use crate::{
  controller::Controller,
  position::Position
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
}

impl Controller for IOController {
  fn get_white_move(&self) -> (Position, Position) {
    if(self.white_human) {
      return get_move();
    } else {
      return get_move(); // TODO: Change later for ai implementation
    }
  }
  fn get_black_move(&self) -> (Position, Position) {
    if(self.black_human) {
      return get_move();
    } else {
      return get_move(); // TODO: Change later for ai implementation
    }
  }
}

fn get_move() -> (Position, Position) {
  loop {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
      Ok(_goes_into_input_above) => {},
      Err(_no_updates_is_fine) => {},
    }
    input = input.trim().to_string();

    let split: Vec<&str> = input.split(",").collect();
    if split.len() == 4 {
      return (
        Position {
          row: split[0].parse::<usize>().unwrap(), 
          column: split[1].parse::<usize>().unwrap()
        }, 
        Position {
          row: split[2].parse::<usize>().unwrap(), 
          column: split[3].parse::<usize>().unwrap()
        }
      )
    }
    println!("Invalid Input");
  }
}