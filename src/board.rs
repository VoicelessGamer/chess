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
   * Returns a copy of the current state of the board pieces
   */
  pub fn get_current_board(&mut self) -> Vec<Vec<Option<Piece>>> {
    return self.board.clone();
  }
}

#[cfg(test)]
mod bridge_tests {
  use crate::{config::{BoardConfig, PieceConfig}, board::Board, position::Position, pieces::piece::Piece};

  /**
   * Tests the move_piece function to make sure that the piece was removed from the previous location and the new location contains a matching piece.
   */
  #[test]
  fn test_move_piece() {
    let board_config = BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);

    let mut current_board = board.get_current_board();

    assert!(current_board[0][0].is_some());
    assert!(current_board[1][1].is_none());

    current_board = board.move_piece(&Position {row: 0, column: 0}, &Position {row: 1, column: 1});

    assert!(current_board[0][0].is_none()); // Should now have moved to position (1,1)
    assert!(current_board[1][1].is_some());

    let piece = current_board[1][1].as_ref().unwrap();
    let mut is_matching = false;
    if let Piece::King(true) = piece { // Check specifically for white king
      is_matching = true;
    }
    assert!(is_matching);
  }

  /**
   * Tests the set_position correctly sets the supplied position with the given piece.
   */
  #[test]
  fn test_set_position_some() {
    let board_config = BoardConfig {
      pieces: vec![],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);

    let current_board = board.set_position(&Position {row: 0, column: 0}, Some(Piece::Bishop(false)));

    assert!(current_board[0][0].is_some());

    let piece = current_board[0][0].as_ref().unwrap();
    let mut is_matching = false;
    if let Piece::Bishop(false) = piece { // Check specifically for black bishop
      is_matching = true;
    }
    assert!(is_matching);
  }

  /**
   * Tests that the set_position function correctly removes the piece from the supplied position.
   */
  #[test]
  fn test_set_position_none() {
    let board_config = BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);

    let mut current_board = board.get_current_board();

    assert!(current_board[0][0].is_some());

    current_board = board.set_position(&Position {row: 0, column: 0}, None);

    assert!(current_board[0][0].is_none());
  }
}