use crate::board::Board;
use crate::config::*;
use crate::pieces::chess_piece::ChessPiece;
use crate::position::Position;

pub struct Game {
  board: Board,
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  white_turn: bool, // true if it is currently white's turn
  incomplete: bool // true while the game is still active (checkmate has NOT occurred)
}

impl Game {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(game_config: GameConfig) -> Self {
    Self { 
      board: Board::new(&game_config.initial_board),
      white_castle: game_config.white_castle,
      black_castle: game_config.black_castle,
      white_turn: game_config.white_turn,
      incomplete: true
    }
  }

  pub fn is_white_turn(&self) -> bool {
    return self.white_turn;
  }

  pub fn is_incomplete(&self) -> bool {
    return self.incomplete;
  }

  pub fn get_current_board(&mut self) -> &Vec<Vec<Option<Box<ChessPiece>>>> {
    return self.board.get_current_board();
  }

  pub fn play_move(&mut self, current_position: Position, new_position: Position) -> &Vec<Vec<Option<Box<ChessPiece>>>> {
    // TODO: this is just temporary
    self.incomplete = false;
    return self.board.move_piece(current_position, new_position);
  }
}