extern crate serde;
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PieceConfig {
  pub piece: String, // A string representation of the piece type. Must be one of the following options: "bishop", "king", "knight", "pawn", "queen", "rook" 
  pub white: bool, // Flag true if the piece is white, false if it is black
  pub row: usize, // Row position on the board
  pub column: usize // Column position on the board
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardConfig {
  pub pieces: Vec<PieceConfig>, // Configuration of each available piece and their position on the board
  pub rows: usize, // The number of rows on the board (chessboard default: 8)
  pub columns: usize // The number of columns on the board (chessboard default: 8)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CastlingConfig {
  pub long_castle: bool, // Whether long castle is available
  pub short_castle: bool // Whether short castle is available
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameConfig {
  pub board: BoardConfig, // The initial state of the board
  pub white_castling: CastlingConfig, // The state of white's castling options
  pub black_castling: CastlingConfig, // The state of black's castling options
  pub white_turn: bool // Determine's who moves first (typically white)
}