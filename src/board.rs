use crate::pieces::piece::Piece;
use crate::pieces::*;
use crate::config::*;

pub struct Board {
  board: Vec<Vec<Option<Box<dyn Piece>>>>
}

impl Board {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(config: &BoardConfig) -> Self {
    let board = vec![vec![None; config.columns]; config.rows];

    /*for piece_config in &config.pieces {
      board[piece_config.x][piece_config.y] = match piece_config.piece.as_str() {
        "bishop" => Some(Box::new(bishop::Bishop::new(piece_config.white)).as_ref()),
        "king" => Some(Box::new(king::King::new(piece_config.white)).as_ref()),
        "knight" => Some(Box::new(knight::Knight::new(piece_config.white)).as_ref()),
        "pawn" => Some(Box::new(pawn::Pawn::new(piece_config.white)).as_ref()),
        "queen" => Some(Box::new(queen::Queen::new(piece_config.white)).as_ref()),
        "rook" => Some(Box::new(rook::Rook::new(piece_config.white)).as_ref())
      }
    }*/
    Self { board }
  }

  /**
   * Function call to place a given piece at a given position
   */
  pub fn move_piece(&mut self, x: usize, y: usize, new_x: usize, new_y: usize) {
    self.board[new_x][new_y] = self.board[x][y];
    self.board[x][y] = None;
  }

  /**
   * Temporarily placed function here until a console interface is in place.
   */
  pub fn print_board(&self) {
    for row in self.board.iter().rev() {
      for col in row.iter() {
        if col.is_some() {
          print!("{} ", col.unwrap().abbreviation());
        } else {
          print!("- ");
        }        
      }
      println!();
    }
  }
}