use crate::{
  pieces::piece::Piece,
  pieces::*,
  move_data::MoveData, 
  position::Position
};


#[derive(Clone)]
pub enum ChessPiece {
  Bishop(Box<bishop::Bishop>),
  King(Box<king::King>),
  Knight(Box<knight::Knight>),
  Pawn(Box<pawn::Pawn>),
  Queen(Box<queen::Queen>),
  Rook(Box<rook::Rook>)
}

impl ChessPiece {
  pub fn is_king(&self) -> bool {
    match self {
        ChessPiece::King(_) => true,
        _ => false
    }
  }
}

impl Piece for ChessPiece {
  fn is_white(&self) -> bool {
    match self {
        ChessPiece::Bishop(bishop) => bishop.is_white(),
        ChessPiece::King(king) => king.is_white(),
        ChessPiece::Knight(knight) => knight.is_white(),
        ChessPiece::Pawn(pawn) => pawn.is_white(),
        ChessPiece::Queen(queen) => queen.is_white(),
        ChessPiece::Rook(rook) => rook.is_white(),
    }
  }

  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData {
    match self {
        ChessPiece::Bishop(bishop) => bishop.get_move_data(origin, board),
        ChessPiece::King(king) => king.get_move_data(origin, board),
        ChessPiece::Knight(knight) => knight.get_move_data(origin, board),
        ChessPiece::Pawn(pawn) => pawn.get_move_data(origin, board),
        ChessPiece::Queen(queen) => queen.get_move_data(origin, board),
        ChessPiece::Rook(rook) => rook.get_move_data(origin, board),
    }
  }
}