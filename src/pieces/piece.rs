pub trait Piece: Clone + 'static {
  fn abbreviation(&self) -> String;
  fn get_moves(&self) -> Vec<(u8, u8)>;
}