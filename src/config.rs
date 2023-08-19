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
  pub white_long_castle: bool, // Whether white can long castle
  pub white_short_castle: bool, // Whether white can short castle
  pub black_long_castle: bool, // Whether black can long castle
  pub black_short_castle: bool, // Whether black can short castle
  pub white_turn: bool
}