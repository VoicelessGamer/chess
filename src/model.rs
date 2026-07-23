use std::collections::HashMap;

use crate::pieces::piece::Piece;

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

#[derive(Debug, Clone)]
pub struct PieceMove {
  pub start: Position,
  pub end: Position,
  pub promotion: Option<String>
}

#[derive(PartialEq, Clone, Debug)]
pub enum State {
  Active,
  BlackWin,
  WhiteWin,
  Draw,
  Error
}

#[derive(Clone)]
pub struct CastlingState {
  pub long_castle: bool, // Is long castling still available
  pub short_castle: bool // Is short castling still available
}

#[derive(Clone)]
pub struct PlayerState {
  pub in_check: bool, // Whether this player's king is in check (currently attacked by opponent)
  pub castling_state: CastlingState, // State of this player's castling options
  pub valid_moves: HashMap<Position, Vec<Position>>, // A map of the current player's valid moves for each piece
  pub last_move: Option<PieceMove>
}

#[derive(Clone)]
pub struct GameState {
  pub state: State, // Current state of play
  pub white_turn: bool, // True if it is currently white's turn
  pub white_state: PlayerState, // The state of the white player
  pub black_state: PlayerState, // The state of the black player
}

pub struct GameStateResult {
  pub board: Vec<Vec<Option<Piece>>>,
  pub game_state: GameState
}