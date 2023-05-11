use crate::board::Board;
use crate::config::*;

pub struct Game {
  board: Board,
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  white_player: u8, // TODO: Come back to this once player classes written
  black_player: u8, // TODO: Come back to this once player classes written
  white_turn: bool // true if it is currently white's turn
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
      white_turn: game_config.white_turn
    }
  }

  pub fn play_move(&mut self) {    
    self.board.print_board();

    //self.board.move_piece(3, 2, 4, 2);

    println!("#########################");
    self.board.print_board();
  }
}