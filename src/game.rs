use crate::{
  board::Board,
  config::*,
  pieces::chess_piece::ChessPiece,
  position::Position,
  player::Player
};

pub enum State {
  ACTIVE,
  BLACK_WIN,
  WHITE_WIN,
  STALEMATE
}

pub struct Game<WP: Player, BP: Player> {
  board: Board,
  white_player: Box<WP>,
  black_player: Box<BP>,
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  white_turn: bool, // true if it is currently white's turn
  state: State // true while the game is still active (checkmate has NOT occurred)
}

impl<WP: Player, BP: Player> Game<WP, BP> {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(game_config: GameConfig, white_player: Box<WP>, black_player: Box<BP>) -> Self {
    Self { 
      board: Board::new(&game_config.initial_board),
      white_player,
      black_player,
      white_castle: game_config.white_castle,
      black_castle: game_config.black_castle,
      white_turn: game_config.white_turn,
      state: State::ACTIVE
    }
  }

  pub fn is_white_turn(&self) -> bool {
    return self.white_turn;
  }

  pub fn get_state(&self) -> &State {
    return &self.state;
  }

  pub fn run(&mut self) {
    self.update_players();
    
    while let State::ACTIVE = self.state {
      let player_move;
      if self.is_white_turn() {
        player_move = self.white_player.get_move();
      } else {
        player_move = self.black_player.get_move();
      }

      // TODO: Validate move here

      self.board.move_piece(player_move.0, player_move.1);

      self.update_players();
    }
  }

  pub fn update_players(&mut self) {
    let current_board = self.board.get_current_board();

    self.white_player.update_state(current_board);
    self.black_player.update_state(current_board);
  }
}