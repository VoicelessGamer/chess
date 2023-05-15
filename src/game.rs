use crate::{
  controller::Controller,
  view::View,
  board::Board,
  config::*,
  pieces::chess_piece::ChessPiece,
  position::Position
};

pub enum State {
  ACTIVE,
  BLACK_WIN,
  WHITE_WIN,
  STALEMATE
}

pub struct Game<C: Controller, V: View> {
  controller: C,
  view: V,
  board: Board,
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  white_turn: bool, // true if it is currently white's turn
  state: State // true while the game is still active (checkmate has NOT occurred)
}

impl<C: Controller, V: View> Game<C, V> {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(controller: C, view: V, game_config: GameConfig) -> Self {
    Self {
      controller,
      view,
      board: Board::new(&game_config.initial_board),
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
        player_move = self.controller.get_white_move();
      } else {
        player_move = self.controller.get_black_move();
      }

      // TODO: Validate move here

      self.board.move_piece(player_move.0, player_move.1);

      self.update_players();
    }
  }

  pub fn update_players(&mut self) {
    let current_board = self.board.get_current_board();

    self.view.update_state(current_board);
  }
}