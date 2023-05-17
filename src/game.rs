use std::collections::{ HashMap, HashSet };

use crate::{
  controller::Controller,
  view::View,
  board::Board,
  config::*,
  player_move::PlayerMove,
  position::Position, 
  pieces::{
    chess_piece::ChessPiece, 
    piece::Piece
  }, 
  move_data::MoveData
};

pub enum State {
  ACTIVE,
  BLACK_WIN,
  WHITE_WIN,
  STALEMATE,
  ERROR
}

pub struct Game<C: Controller, V: View> {
  controller: C,
  view: V,
  board: Board,
  white_turn: bool, // true if it is currently white's turn
  state: State, // true while the game is still active (checkmate has NOT occurred)
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  player_check: bool // Whether the current player's king is in check (updated for the next player after each move)
}

impl<C: Controller, V: View> Game<C, V> {
  /**
   * Initialises a standard chess game
   */
  pub fn new(controller: C, view: V, game_config: GameConfig) -> Self {
    // TODO: Add validation of the game config, checking for position boundaries single king per side
    Self {
      controller,
      view,
      board: Board::new(&game_config.initial_board),
      white_turn: game_config.white_turn,
      state: State::ACTIVE,
      white_castle: game_config.white_castle,
      black_castle: game_config.black_castle,
      player_check: false
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
        self.evaluate_board(&current_board);
  
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
    let pos = Position { row: player_move.current.row, column: player_move.current.column };
    if !current_piece.get_move_data(pos).attacks.contains(&player_move.target) {
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

  /**
   * Evaluates the current position of the board, searching for a check or a
   * checkmate on the opposing player. This function will update self.state
   * and self.player_check accordingly.
   */
  fn evaluate_board(&mut self, board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
    let mut opposing_king: Option<MoveData> = None;     // Move data for the opposing player's king
    let mut attacked_positions: Vec<Position> = vec![]; // List of all attacked positions for the current player
    let mut opposing_move_data: Vec<MoveData> = vec![]; // List of the move data for each piece of the opposing player
    let mut players_move_data: Vec<MoveData> = vec![];  // List of the move data for each piece of the current player
    let mut checking_pieces: Vec<MoveData> = vec![];    // List of the move data for the current player's piece which are checking the opposing king

    self.player_check = false;

    // Retrieve all the positional information to determine check state
    // TODO: Lots of cloning of the move data in this loop, must check the effect on performance
    for i in 0..board.len() {
      let row = &board[i];
      for j in 0..row.len() {
        let piece = &row[j];
        match piece {
          None => continue,
          Some(chess_piece) => {
            let position = Position {row: i, column: j};
            let move_data = chess_piece.get_move_data(position);

            if (self.white_turn && !chess_piece.is_white()) || // White's turn, this is black piece or
                (!self.white_turn && chess_piece.is_white()) { // Black's turn, this is white piece
              // This part of the if-else statement is for retrieving the opposing player's data

              // Check if this piece is the king and gather data to separate variable
              if chess_piece.is_king() {
                opposing_king = Some(move_data.clone());
              }
              opposing_move_data.push(move_data);
            } else {
              // This part of the if-else statement is for retrieving the current player's data

              attacked_positions.extend(move_data.clone().attacks);
              players_move_data.push(move_data.clone());
              if move_data.checking {
                checking_pieces.push(move_data);
                self.player_check = true;
              }              
            }
          }
        }
      }
    }

    // If the opposing king was not found or if there is more than one checking piece then there has been an error in gameplay/logic, cannot continue
    if opposing_king.is_none() || checking_pieces.len() > 2 {
      self.state = State::ERROR;
      return;
    }

    // If opposing king is not check then return
    if !self.player_check {
      return;
    }

    /*
     * From this point on all checks are specifically for checkmate
     */

    let king_move_data = opposing_king.unwrap();

    // Check if the king can move out of check
    for position in king_move_data.attacks {
      if !attacked_positions.contains(&position) {
        return;
      }
    }

    // Check for valid blocks or takes of the checking pieces
    // TODO: Finish the checkmate checks
    if checking_pieces.len() == 1 {
      /* If one checker, 
      * can it be taken by standard piece          [NEED:: A VEC<MOVE_DATA> OF ALL OPPOSING PIECES]
      * can it be taken by king and not defended   [NEED:: A VEC<MOVE_DATA> OF ALL CURRENT PIECES]
      * can it be blocked                          [NEED:: A VEC<MOVE_DATA> OF ALL OPPOSING PIECES] 
      */
    } else if checking_pieces.len() == 2 {
      /* If 2 checkers,
       * are either undefended and can be taken by king   [NEED:: A VEC<MOVE_DATA> OF ALL CURRENT PIECES]
       */
    }
  }
}