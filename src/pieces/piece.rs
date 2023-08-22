#[derive(PartialEq, Clone)]
pub enum Piece {
  Bishop(bool),
  King(bool),
  Knight(bool),
  Pawn(bool),
  Queen(bool),
  Rook(bool)
}

impl Piece {
  pub fn is_king(&self) -> bool {
    match self {
      Piece::King(_) => true,
      _ => false
    }
  }
  pub fn is_white(&self) -> bool {
    match self {
      Piece::Bishop(is_white) => *is_white,
      Piece::King(is_white) => *is_white,
      Piece::Knight(is_white) => *is_white,
      Piece::Pawn(is_white) => *is_white,
      Piece::Queen(is_white) => *is_white,
      Piece::Rook(is_white) => *is_white,
    }
  }
}