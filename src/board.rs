use crate::pieces::chess_piece::ChessPiece;
use crate::pieces::piece::Piece;
use crate::pieces::*;
use crate::config::*;

pub struct Board {
  board: Vec<Vec<Option<Box<ChessPiece>>>>
}

impl Board {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(config: &BoardConfig) -> Self {
    let mut board: Vec<Vec<Option<Box<ChessPiece>>>> = vec![vec![None; config.columns]; config.rows];

    for piece_config in &config.pieces {
      board[piece_config.row][piece_config.column] = match piece_config.piece.as_str() {
        "bishop" => Some(Box::new(ChessPiece::Bishop(Box::new(bishop::Bishop::new(piece_config.row, piece_config.column, piece_config.white))))),
        "king"   => Some(Box::new(ChessPiece::King(Box::new(king::King::new(      piece_config.row, piece_config.column, piece_config.white))))),
        "knight" => Some(Box::new(ChessPiece::Knight(Box::new(knight::Knight::new(piece_config.row, piece_config.column, piece_config.white))))),
        "pawn"   => Some(Box::new(ChessPiece::Pawn(Box::new(pawn::Pawn::new(      piece_config.row, piece_config.column, piece_config.white))))),
        "queen"  => Some(Box::new(ChessPiece::Queen(Box::new(queen::Queen::new(   piece_config.row, piece_config.column, piece_config.white))))),
        "rook"   => Some(Box::new(ChessPiece::Rook(Box::new(rook::Rook::new(      piece_config.row, piece_config.column, piece_config.white))))),
        _        => unimplemented!()
      };
    };

    Self { board }
  }

  /**
   * Function call to place a given piece at a given position
   */
  pub fn move_piece(&mut self, x: usize, y: usize, new_x: usize, new_y: usize) {
    let chess_piece = self.board[x][y].take();
    self.board[x][y] = None;
    self.board[new_x][new_y] = chess_piece;
  }

  /**
   * Temporarily placed function here until a console interface is in place.
   */
  pub fn print_board(&self) {
    for row in self.board.iter().rev() {
      for col in row.iter() {
        if col.is_some() {
          print!("{} ", col.as_ref().unwrap().abbreviation());
        } else {
          print!("- ");
        }        
      }
      println!();
    }
  }
}