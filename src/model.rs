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

#[derive(Debug, Clone)]
pub struct CastlingState {
  /// Is long castling still available
  pub long_castle: bool,
  /// Is short castling still available
  pub short_castle: bool 
}

#[derive(Debug, Clone)]
pub struct PlayerState {
  /// Whether this player's king is in check (currently attacked by opponent)
  pub in_check: bool, 
  /// State of this player's castling options
  pub castling_state: CastlingState,
  /// A map of the current player's valid moves for each piece
  pub valid_moves: HashMap<Position, Vec<Position>>,
  /// The last move made by this player
  pub last_move: Option<PieceMove>
}

#[derive(Debug, Clone)]
pub struct GameState {
  /// Current state of play
  pub state: State, 
  /// True if it is currently white's turn
  pub white_turn: bool, 
  /// The state of the white player
  pub white_state: PlayerState, 
  /// The state of the black player
  pub black_state: PlayerState, 
}

#[derive(Debug)]
pub struct GameStateResult {
  /// The current position of all the pieces on the board
  pub board: Vec<Vec<Option<Piece>>>,
  // The current full state data of the game
  pub game_state: GameState
}