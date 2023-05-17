#[derive(Eq, Hash, Clone, Debug)]
pub struct Position {
  pub row: usize,
  pub column: usize
}

impl PartialEq for Position {
  fn eq(&self, other: &Self) -> bool {
    if self.row != other.row {
      return false;
    }
    if self.column != other.column {
      return false;
    }
    return true;
  }
}