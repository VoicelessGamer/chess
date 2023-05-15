use crate::{
  view::View,
  pieces::chess_piece::ChessPiece
};

pub struct IOView {
  pub use_unicode: bool
}

impl View for IOView {
  fn update_state(&mut self, board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
    if self.use_unicode {
      self.print_unicode_board(board);
    } else {
      self.print_letter_board(board);
    }
  }
}

impl IOView {
  fn print_unicode_board(&mut self, board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
    println!("   0 1 2 3 4 5 6 7");

    let mut row_index = 7;

    for row in board.iter().rev() {
      print!("{}  ", row_index);
      for col in row.iter() {
        if col.is_some() {
          let piece = col.as_ref().unwrap();
          match &**piece {
            ChessPiece::Bishop(bishop) => {
                if bishop.white { print!("♗ ") } else { print!("♝ ") };
              },
              ChessPiece::King(king) => {
                if king.white { print!("♔ ") } else { print!("♚ ") };
              },
              ChessPiece::Knight(knight) => {
                if knight.white { print!("♘ ") } else { print!("♞ ") };
              },
              ChessPiece::Pawn(pawn) => {
                if pawn.white { print!("♙ ") } else { print!("♟︎ ") };
              },
              ChessPiece::Queen(queen) => {
                if queen.white { print!("♕ ") } else { print!("♛ ") };
              },
              ChessPiece::Rook(rook) => {
                if rook.white { print!("♖ ") } else { print!("♜ ") };
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
    println!("#################################");
    println!("");
  }

  fn print_letter_board(&mut self, board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
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
}