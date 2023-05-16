use crate::{
  controller::Controller,
  view::View,
  board::Board,
  config::*,
  player_move::PlayerMove,
  position::Position, 
  pieces::{chess_piece::ChessPiece, piece::Piece}
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
  player_check: bool, // Whether the current player's king is in check (updated for the next player after each move)
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
      player_check: false,
      white_turn: game_config.white_turn,
      state: State::ACTIVE
    }
  }

  /**
   * Returns the current state of the game
   */
  #[allow(dead_code)]
  pub fn get_state(&self) -> &State {
    return &self.state;
  }

  /**
   * This is the entrypoint for the main logic of the game. Once called, this
   * function will process the moves provided by the controller, handle validation
   * and update the view with the changes to the board.
   */
  pub fn run(&mut self) {
    // Initialise the view for the players
    let mut current_board = self.board.get_current_board();
    self.view.update_state(&current_board);
    
    // Loop the turn based logic until there is an outcome for the game
    while let State::ACTIVE = self.state {
      let player_move = self.controller.get_move(self.white_turn);

      /*
       * Validate the chosen move
       * NOTE: the board is taken as a mutable reference here as it is not read
       * from again until the move has been performed. This has been done so 
       * that a second copy of the board does not need to be made.
       */
      let is_valid = self.validate_move(&player_move, &mut current_board);

      if is_valid {
        // The move is valid, make the move on the board and update the players with the current board state
        current_board = self.board.move_piece(player_move.current, player_move.target);

        // TODO: Update the player_check variable here based on the board change
  
        self.view.update_state(&current_board);

        // Swap the active player
        self.white_turn = !self.white_turn;
      }
    }
  }

  /**
   * Validate the player move against the current board using standard chess 
   * rules. Returns true if the move can be made.
   * 
   * NOTE: This function takes a mutable reference to the board so that
   * validations can be performed in the modified position. Any reference
   * to the supplied board after this function call will be referencing a
   * modified board.
   */
  fn validate_move(&self, player_move: &PlayerMove, board: &mut Vec<Vec<Option<Box<ChessPiece>>>>) -> bool {
    // Check the chosen piece exists and is not an empty space on the board
    let current_piece = match &board[player_move.current.row][player_move.current.column] {
      None => return false,
      Some(piece) => piece
    };

    // Check the chosen piece belongs to the current active player
    if (self.white_turn && !current_piece.is_white()) ||
        (!self.white_turn && current_piece.is_white()) {
      return false;
    }

    // Check the target position is a valid position for that piece
    if !current_piece.get_moves().contains(&player_move.target) {
      return false;
    }

    // Modify the board to represent the board after the move, so that check validations can be performed
    let chess_piece = board[player_move.current.row][player_move.current.column].take();
    board[player_move.current.row][player_move.current.column] = None;
    board[player_move.target.row][player_move.target.column] = chess_piece;

    // TODO: If the current player's king is in check, validate this move would resolve the check

    // TODO: Check if the move would cause the active player's king to be under attack

    return true;
  }
}