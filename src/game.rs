use crate::board::Board;
use crate::config::*;
use crate::pieces::chess_piece::ChessPiece;

pub struct Game {
  board: Board,
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  white_player: u8, // TODO: Come back to this once player classes written
  black_player: u8, // TODO: Come back to this once player classes written
  white_turn: bool, // true if it is currently white's turn
  complete: bool // true when the game is complete (checkmate occurs)
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
      white_player: 0,
      black_player: 1,
      white_turn: game_config.white_turn,
      complete: false
    }
  }

  pub fn get_current_board(&mut self) -> &Vec<Vec<Option<Box<ChessPiece>>>> {
    return self.board.get_current_board();
  }

  pub fn play_move(&mut self) -> &Vec<Vec<Option<Box<ChessPiece>>>> {
    return self.board.move_piece(1, 2, 3, 2);
  }
}