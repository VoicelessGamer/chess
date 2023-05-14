use crate::pieces::chess_piece::ChessPiece;
use crate::pieces::*;
use crate::config::*;
use crate::position::Position;

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
  pub fn move_piece(&mut self, current_position: Position, new_position: Position) -> &Vec<Vec<Option<Box<ChessPiece>>>> {
    let chess_piece = self.board[current_position.row][current_position.column].take();
    self.board[current_position.row][current_position.column] = None;
    self.board[new_position.row][new_position.column] = chess_piece;

    return self.get_current_board();
  }

  /**
   * Returns a reference to the current state of the board pieces
   */
  pub fn get_current_board(&mut self) -> &Vec<Vec<Option<Box<ChessPiece>>>> {
    return &self.board;
  }
}