use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct King {
  white: bool
}

impl King {
  pub fn new(white: bool) -> King {
    King { white }
  }
}

impl Piece for King {
  fn abbreviation(&self) -> String {
    return String::from("K");
  }

  fn get_moves(&self) -> Vec<(u8, u8)> {
    todo!()
  }
}
