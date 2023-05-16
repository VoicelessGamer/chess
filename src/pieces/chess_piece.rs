use crate::{
  pieces::piece::Piece,
  pieces::*,
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

  fn get_moves(&self) -> Vec<Position> {
    match self {
        ChessPiece::Bishop(bishop) => bishop.get_moves(),
        ChessPiece::King(king) => king.get_moves(),
        ChessPiece::Knight(knight) => knight.get_moves(),
        ChessPiece::Pawn(pawn) => pawn.get_moves(),
        ChessPiece::Queen(queen) => queen.get_moves(),
        ChessPiece::Rook(rook) => rook.get_moves(),
    }
  }
}