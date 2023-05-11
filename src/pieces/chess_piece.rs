use crate::pieces::piece::Piece;
use crate::pieces::*;

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
  fn get_position(&self) -> (usize, usize) {
    match self {
        ChessPiece::Bishop(bishop) => bishop.get_position(),
        ChessPiece::King(king) => king.get_position(),
        ChessPiece::Knight(knight) => knight.get_position(),
        ChessPiece::Pawn(pawn) => pawn.get_position(),
        ChessPiece::Queen(queen) => queen.get_position(),
        ChessPiece::Rook(rook) => rook.get_position(),
    }
  }

  fn abbreviation(&self) -> String {
    match self {
        ChessPiece::Bishop(bishop) => bishop.abbreviation(),
        ChessPiece::King(king) => king.abbreviation(),
        ChessPiece::Knight(knight) => knight.abbreviation(),
        ChessPiece::Pawn(pawn) => pawn.abbreviation(),
        ChessPiece::Queen(queen) => queen.abbreviation(),
        ChessPiece::Rook(rook) => rook.abbreviation(),
    }
  }

  fn get_moves(&self) -> Vec<(usize, usize)> {
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