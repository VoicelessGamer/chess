use crate::{
  view::View,
  pieces::piece::Piece, game::State
};

pub struct IOView {
  pub use_unicode: bool
}

impl View for IOView {
  fn update_state(&mut self, board: &Vec<Vec<Option<Piece>>>, game_state: State) {
    if self.use_unicode {
      self.print_unicode_board(board, game_state);
    } else {
      self.print_letter_board(board, game_state);
    }
  }
}

impl IOView {
  fn print_unicode_board(&mut self, board: &Vec<Vec<Option<Piece>>>, game_state: State) {
    println!("");
    println!("   0 1 2 3 4 5 6 7");

    let mut row_index = 7;

    for row in board.iter().rev() {
      print!("{}  ", row_index);
      for col in row.iter() {
        if col.is_some() {
          let piece = col.as_ref().unwrap();
          match &piece {
            Piece::Bishop(is_white) => {
              if *is_white { print!("♗ ") } else { print!("♝ ") };
            },
            Piece::King(is_white) => {
              if *is_white{ print!("♔ ") } else { print!("♚ ") };
            },
            Piece::Knight(is_white) => {
              if *is_white { print!("♘ ") } else { print!("♞ ") };
              },
            Piece::Pawn(is_white) => {
              if *is_white { print!("♙ ") } else { print!("♟︎ ") };
            },
            Piece::Queen(is_white) => {
              if *is_white { print!("♕ ") } else { print!("♛ ") };
            },
            Piece::Rook(is_white) => {
              if *is_white { print!("♖ ") } else { print!("♜ ") };
            },
          }
          
        } else {
          print!("- ");
        }        
      }
      println!(" {}", row_index);
      row_index -= 1;
    }

    println!("   0 1 2 3 4 5 6 7");

    // TODO: clear console between updates
    println!("");
    if game_state.white_turn {
      println!("Turn: White");
    } else {
      println!("Turn: Black");
    }
    println!("Game State: {:?}", game_state.game_state);
    println!("In Check? {:?}", game_state.in_check);
    println!("");
    println!("#################################");
    println!("");
  }

  fn print_letter_board(&mut self, board: &Vec<Vec<Option<Piece>>>, _game_state: State) {
    println!("    0  1  2  3  4  5  6  7");

    let mut row_index = 7;

    for row in board.iter().rev() {
      print!("{}  ", row_index);
      for col in row.iter() {
        if col.is_some() {
          let piece = col.as_ref().unwrap();
          match &piece {
            Piece::Bishop(is_white) => {
                if *is_white { print!("w") } else { print!("b") };
                print!("B ");
              },
              Piece::King(is_white) => {
                if *is_white { print!("w") } else { print!("b") };
                print!("K ");
              },
              Piece::Knight(is_white) => {
                if *is_white { print!("w") } else { print!("b") };
                print!("N ");
              },
              Piece::Pawn(is_white) => {
                if *is_white { print!("w") } else { print!("b") };
                print!("P ");
              },
              Piece::Queen(is_white) => {
                if *is_white { print!("w") } else { print!("b") };
                print!("Q ");
              },
              Piece::Rook(is_white) => {
                if *is_white { print!("w") } else { print!("b") };
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
}