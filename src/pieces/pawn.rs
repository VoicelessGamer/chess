use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Pawn {
  white: bool,
}

impl Pawn {
  pub fn new(white: bool) -> Pawn {
    Pawn { white }
  }
}

impl Piece for Pawn {
  fn abbreviation(&self) -> String {
    return String::from("P");
  }

  fn get_moves(&self) -> Vec<(u8, u8)> {
    todo!()
  }
}
