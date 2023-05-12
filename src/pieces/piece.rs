pub trait Piece: Clone + 'static {
  fn get_position(&self) -> (usize, usize);
  fn get_moves(&self) -> Vec<(usize, usize)>;
}