use crate::pieces::piece::Piece;

#[derive(Clone)]
pub struct Bishop {
  white: bool,
}

impl Bishop {
  pub fn new(white: bool) -> Bishop {
    Bishop { white }
  }
}

impl Piece for Bishop {
  fn abbreviation(&self) -> String {
    return String::from("B");
  }

  fn get_moves(&self) -> Vec<(u8, u8)> {
    todo!()
  }
}
