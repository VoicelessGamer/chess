use std::io;

use crate::{player::Player, position::Position};

pub struct ComputerPlayer {
  pub white: bool,
  pub difficulty: u32
}

impl Player for ComputerPlayer {
  fn get_move(&mut self) -> (Position, Position) {
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
}