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

/**
 * Returns a new Piece for the matching promotion id
 */
pub fn get_promotion_piece(promotion_id: &String, is_white: bool) -> Option<Piece> {
  match promotion_id.as_str() {
    "B" => Some(Piece::Bishop(is_white)),
    "N" => Some(Piece::Knight(is_white)),
    "Q" => Some(Piece::Queen(is_white)),
    "R" => Some(Piece::Rook(is_white)),
    _ => None
  }
}