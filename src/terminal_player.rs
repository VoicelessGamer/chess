use std::io;

use crate::{
  player::Player, 
  position::Position, 
  pieces::chess_piece::ChessPiece
};

pub struct TerminalPlayer {
  pub white: bool
}

impl Player for TerminalPlayer {
  fn update_state(&mut self, board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
    println!("    0  1  2  3  4  5  6  7");

    let mut row_index = 7;

    for row in board.iter().rev() {
      print!("{}  ", row_index);
      for col in row.iter() {
        if col.is_some() {
          let piece = col.as_ref().unwrap();
          match &**piece {
              ChessPiece::Bishop(bishop) => {
                if bishop.white { print!("w") } else { print!("b") };
                print!("B ");
              },
              ChessPiece::King(king) => {
                if king.white { print!("w") } else { print!("b") };
                print!("K ");
              },
              ChessPiece::Knight(knight) => {
                if knight.white { print!("w") } else { print!("b") };
                print!("N ");
              },
              ChessPiece::Pawn(pawn) => {
                if pawn.white { print!("w") } else { print!("b") };
                print!("P ");
              },
              ChessPiece::Queen(queen) => {
                if queen.white { print!("w") } else { print!("b") };
                print!("Q ");
              },
              ChessPiece::Rook(rook) => {
                if rook.white { print!("w") } else { print!("b") };
                print!("R ");
              },
          }
          
        } else {
          print!("-- ");
        }        
      }
      println!(" {}", row_index);
      row_index -= 1;
    }

    println!("    0  1  2  3  4  5  6  7");

    // TODO: clear console between updates
    println!("");
    println!("#################################");
    println!("");
  }

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