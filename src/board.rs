use crate::pieces::piece::Piece;
use crate::config::*;
use crate::position::Position;

#[derive(Clone)]
pub struct Board {
  board: Vec<Vec<Option<Piece>>>
}

impl Board {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(config: &BoardConfig) -> Self {
    let mut board: Vec<Vec<Option<Piece>>> = vec![vec![None; config.columns]; config.rows];

    for piece_config in &config.pieces {
      board[piece_config.row][piece_config.column] = match piece_config.piece.as_str() {
        "bishop" => Some(Piece::Bishop(piece_config.white)),
        "king"   => Some(Piece::King(piece_config.white)),
        "knight" => Some(Piece::Knight(piece_config.white)),
        "pawn"   => Some(Piece::Pawn(piece_config.white)),
        "queen"  => Some(Piece::Queen(piece_config.white)),
        "rook"   => Some(Piece::Rook(piece_config.white)),
        _        => unimplemented!()
      };
    };

    Self { board }
  }

  /**
   * Function call to place a given piece at a given position
   */
  pub fn move_piece(&mut self, current_position: &Position, new_position: &Position) -> Vec<Vec<Option<Piece>>> {
    let chess_piece = self.board[current_position.row][current_position.column].take();
    self.board[current_position.row][current_position.column] = None;
    self.board[new_position.row][new_position.column] = chess_piece;

    return self.get_current_board();
  }

  /**
   * Function call to set a given position on the board to a given piece
   */
  pub fn set_position(&mut self, position: &Position, piece: Option<Piece>) -> Vec<Vec<Option<Piece>>> {
    self.board[position.row][position.column] = piece;

    return self.get_current_board();
  }

  /**
   * Function call to clear a given position on the board
   */
  pub fn clear_position(&mut self, position: &Position) -> Vec<Vec<Option<Piece>>> {
    self.board[position.row][position.column] = None;

    return self.get_current_board();
  }

  /**
   * Returns a copy of the current state of the board pieces
   */
  pub fn get_current_board(&mut self) -> Vec<Vec<Option<Piece>>> {
    return self.board.clone();
  }
}