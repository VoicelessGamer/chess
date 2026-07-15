use crate::config;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct PieceConfig {
  pub piece: String, // A string representation of the piece type. Must be one of the following options: "bishop", "king", "knight", "pawn", "queen", "rook" 
  pub white: bool, // Flag true if the piece is white, false if it is black
  pub row: usize, // Row position on the board
  pub column: usize // Column position on the board
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct BoardConfig {
  pub pieces: Vec<PieceConfig>, // Configuration of each available piece and their position on the board
  pub rows: usize, // The number of rows on the board (chessboard default: 8)
  pub columns: usize // The number of columns on the board (chessboard default: 8)
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct CastlingConfig {
  pub long_castle: bool, // Whether long castle is available
  pub short_castle: bool // Whether short castle is available
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct GameConfig {
  pub board: BoardConfig, // The initial state of the board
  pub white_castling: CastlingConfig, // The state of white's castling options
  pub black_castling: CastlingConfig, // The state of black's castling options
  pub white_turn: bool // Determine's who moves first (typically white)
}

impl Default for GameConfig {
  fn default() -> Self {
    Self {
      board: config::BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 3, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 4, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 5, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 6, row: 1},
          PieceConfig {piece: String::from("pawn"), white: true, column: 7, row: 1},
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
          PieceConfig {piece: String::from("knight"), white: true, column: 1, row: 0},
          PieceConfig {piece: String::from("bishop"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("queen"), white: true, column: 3, row: 0},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("bishop"), white: true, column: 5, row: 0},
          PieceConfig {piece: String::from("knight"), white: true, column: 6, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},

          PieceConfig {piece: String::from("pawn"), white: false, column: 0, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 1, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 2, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 3, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 4, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 5, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 6, row: 6},
          PieceConfig {piece: String::from("pawn"), white: false, column: 7, row: 6},
          PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7},
          PieceConfig {piece: String::from("knight"), white: false, column: 1, row: 7},
          PieceConfig {piece: String::from("bishop"), white: false, column: 2, row: 7},
          PieceConfig {piece: String::from("queen"), white: false, column: 3, row: 7},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
          PieceConfig {piece: String::from("bishop"), white: false, column: 5, row: 7},
          PieceConfig {piece: String::from("knight"), white: false, column: 6, row: 7},
          PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: config::CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: config::CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    }
  }
}