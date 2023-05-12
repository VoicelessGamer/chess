use crate::{game::Game, pieces::chess_piece::ChessPiece};

pub struct ConsoleController {
  game: Game
}

impl ConsoleController {
  pub fn new(game: Game) -> Self {
    Self { 
      game
    }
  }

  pub fn run(&mut self) {
    let current_board = self.game.get_current_board();
    self.print_board(current_board);

    println!("");
    println!("#########################");
    println!("");
    //self.game.play_move();
  }

  /**
   * Prints the current state of the board to the console output
   */
  pub fn print_board(&mut self, board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
    for row in board.iter().rev() {
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
      println!();
    }
  }
}