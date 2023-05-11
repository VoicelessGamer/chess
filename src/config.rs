pub struct PieceConfig {
  pub piece: String,
  pub white: bool,
  pub row: usize,
  pub column: usize
}

pub struct BoardConfig {
  pub pieces: Vec<PieceConfig>,
  pub rows: usize,
  pub columns: usize
}

pub struct GameConfig {
  pub initial_board: BoardConfig,
  pub white_castle: bool, // Whether white can castle
  pub black_castle: bool, // Whether black can castle
  pub white_turn: bool
}