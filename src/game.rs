use std::collections::HashSet;

use crate::{
  controller::Controller,
  view::View,
  board::Board,
  config::*,
  player_move::PlayerMove,
  position::Position, 
  pieces::{piece::*, self},
  move_data::MoveData
};

#[derive(Clone, Debug)]
pub enum GameState {
  Active,
  BlackWin,
  WhiteWin,
  Stalemate,
  Error
}

#[derive(Clone)]
pub struct State {
  pub white_turn: bool, // true if it is currently white's turn
  pub game_state: GameState, // Current state of play
  pub _white_castle: bool, // Whether white can still castle
  pub _black_castle: bool, // Whether black can still castle
  pub in_check: bool // Whether the current player's king is in check (updated for the next player after each move)
}

pub struct Game<C: Controller, V: View> {
  controller: C,
  view: V,
  board: Board,
  state: State, // Holds the current state of the game
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
      state: State{
        white_turn: game_config.white_turn,
        game_state: GameState::Active,
        _white_castle: game_config.white_castle,
        _black_castle: game_config.black_castle,
        in_check: false
      }
    }
  }

  /**
   * This is the entrypoint for the main logic of the game. Once called, this
   * function will process the moves provided by the controller, handle validation
   * and update the view with the changes to the board.
   */
  pub fn run(&mut self) {
    // Initialise the view for the players
    let mut current_board = self.board.get_current_board();

    self.view.update_state(&current_board, self.state.clone());
    
    // Loop the turn based logic until there is an outcome for the game
    while let GameState::Active = self.state.game_state {
      let player_move = self.controller.get_move(self.state.white_turn);

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

        // Evaluate the new board and update the game state
        self.update_game_state(&current_board);

        // Swap the active player
        self.state.white_turn = !self.state.white_turn;
  
        // Update the player's views
        self.view.update_state(&current_board, self.state.clone());
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
  fn validate_move(&self, player_move: &PlayerMove, board: &mut Vec<Vec<Option<Piece>>>) -> bool {
    // Check the chosen piece exists and is not an empty space on the board
    let current_piece = match &board[player_move.current.row][player_move.current.column] {
      None => return false,
      Some(piece) => piece
    };

    // Check the chosen piece belongs to the current active player
    if (self.state.white_turn && !current_piece.is_white()) ||
        (!self.state.white_turn && current_piece.is_white()) {
      return false;
    }

    // Check the target position is a valid position for that piece
    let pos = Position { row: player_move.current.row, column: player_move.current.column };
    if !get_move_data(pos, board).valid_moves.contains(&player_move.target) {
      return false;
    }

    // Modify the board to represent the board after the move, so that check validations can be performed
    let chess_piece = board[player_move.current.row][player_move.current.column].take();
    board[player_move.current.row][player_move.current.column] = None;
    board[player_move.target.row][player_move.target.column] = chess_piece;

    // Check if the move would cause the active player's king to be under attack
    // This also resolves the check for if this move would resolve any existing check
    let mut is_checking: bool = false;
    'outer:  for i in 0..board.len() {
      let row = &board[i];
      for j in 0..row.len() {
        let piece = &row[j];
        is_checking = match piece {
          None => false,
          Some(chess_piece) => {
            if (self.state.white_turn && !chess_piece.is_white()) || // White's turn, this is black piece or
                (!self.state.white_turn && chess_piece.is_white()) { // Black's turn, this is white piece

              let position = Position {row: i, column: j};
              let move_data = get_move_data(position, board);

              if move_data.checking_path.is_some() {
                true
              } else {
                false
              }
            } else {
              false
            }
          }
        };
        if is_checking {
          break 'outer;
        }
      }
    }

    if is_checking {
      return false;
    }

    return true;
  }

  /**
   * Evaluates the current position of the board, searching for a check or a
   * checkmate on the opposing player. This function will return the state of 
   * check for the opposing player and the new game state.
   */
  fn update_game_state(&mut self, board: &Vec<Vec<Option<Piece>>>) {
    let mut opposing_king: Option<MoveData> = None;     // Move data for the opposing player's king
    let mut attacked_positions: Vec<Position> = vec![]; // List of all attacked positions for the current player
    let mut opposing_move_data: Vec<MoveData> = vec![]; // List of the move data for each piece of the opposing player
    let mut players_move_data: Vec<MoveData> = vec![];  // List of the move data for each piece of the current player
    let mut checking_pieces: Vec<MoveData> = vec![];    // List of the move data for the current player's piece which are checking the opposing king
    let mut defended_player_pieces: HashSet<Position> = HashSet::new(); // List of all the current player's pieces which are defended by another piece
    let mut pinned_positions: HashSet<Position> = HashSet::new();       // List of all opposing pieces pinned to the king

    let mut player_check = false;

    // Retrieve all the positional information to determine check state
    // TODO: Lots of cloning of the move data in this loop, must check the effect on performance. May be better to collect all first and second loop to get references
    for i in 0..board.len() {
      let row = &board[i];
      for j in 0..row.len() {
        let piece = &row[j];
        match piece {
          None => continue,
          Some(chess_piece) => {
            let position = Position {row: i, column: j};
            let move_data = get_move_data(position, board);

            if (self.state.white_turn && !chess_piece.is_white()) || // White's turn, this is black piece or
                (!self.state.white_turn && chess_piece.is_white()) { // Black's turn, this is white piece
              // This part of the if-else statement is for retrieving the opposing player's data

              // Check if this piece is the king and gather data to separate variable
              if chess_piece.is_king() {
                opposing_king = Some(move_data);
              } else {
                opposing_move_data.push(move_data);
              }
            } else {
              // This part of the if-else statement is for retrieving the current player's data (current piece matches current player)
              let cloned_data = move_data.clone();
              attacked_positions.extend(cloned_data.attacks);
              defended_player_pieces.extend(cloned_data.defends);
              players_move_data.push(move_data.clone());
              pinned_positions.extend(cloned_data.pins);
              if move_data.checking_path.is_some() {
                checking_pieces.push(move_data);
              }
            }
          }
        }
      }
    }

    // TODO: Need to sort out returning the opponents valid moves

    // If the opposing king was not found or if there is more than one checking piece then there has been an error in gameplay/logic, cannot continue
    if opposing_king.is_none() || checking_pieces.len() > 2 {
      self.state.in_check = player_check;
      self.state.game_state = GameState::Error;
      return;
    }

    // If checking_pieces is not empty then the opposing played is currently in check, not accounting for stalemate or checkmate
    if !checking_pieces.is_empty() {
      player_check = true;
    }

    let mut opponent_can_move = false;
    let mut one_checker_valid_defend = false; // Used to determine if the opponent has a standard piece that can take or block a single checking piece

    // For all opposing pieces that are pinned by current player, wipe the valid moves
    for mut move_data in opposing_move_data {
      if pinned_positions.contains(&move_data.position) {
        move_data.valid_moves = vec![];
      } else {
        opponent_can_move = true;

        // Determine a standard piece can block the check or capture the checker
        if !one_checker_valid_defend && checking_pieces.len() == 1 {
          let checking_piece = &checking_pieces[0];
          let checking_path = checking_piece.checking_path.as_ref().unwrap(); // Checking pieces should always have Some(checking_path)
          if checking_path.is_empty() {
            continue;
          }
          let position = &checking_piece.position;
          // Checking all the valid move positions by the opposing piece
          for attacked_position in &move_data.valid_moves {
            // If the checking piece can be captured 
            if attacked_position == position || checking_path.contains(attacked_position) {
              one_checker_valid_defend = true;
              break;
            }
          }
        }
      }
    }

    // For the opposing king remove any moves that are not valid based on the current players' pieces
    let mut king_valid_moves = vec![];
    for position in &opposing_king.as_ref().unwrap().valid_moves {
      // Adding the valid moves (non defended positions) to a separate vec
      if !attacked_positions.contains(position) && !defended_player_pieces.contains(position) {
        king_valid_moves.push(position.clone());
        opponent_can_move = true;
      }
    }
    // Overwriting the kings valid moves
    opposing_king.as_mut().unwrap().valid_moves = king_valid_moves;

    // TODO: Gather all valid moves for the opponent here, set in state

    // If not check and no opposing piece has a valid move then stalemate
    if !player_check && !opponent_can_move {
      self.state.in_check = player_check;
      self.state.game_state = GameState::Stalemate;
      return;
    } else if player_check && opposing_king.as_ref().unwrap().valid_moves.is_empty() && !one_checker_valid_defend {
      // Opposing player in checkmate, change state to end game
      if self.state.white_turn {
        self.state.in_check = player_check;
        self.state.game_state = GameState::WhiteWin;
      } else {
        self.state.in_check = player_check;
        self.state.game_state = GameState::BlackWin;
      }
      return;
    }

    self.state.in_check = player_check;
    self.state.game_state = GameState::Active;
    return;
  }
}

/**
 * Get the relevant move data based on the Piece type in the given position
 */
fn get_move_data(position: Position, board: &Vec<Vec<Option<Piece>>>) -> MoveData {
  match &board[position.row][position.column] {
    Some(piece) => {
      match piece {
        Piece::Bishop(_) => pieces::bishop::get_bishop_move_data(position, board),
        Piece::King(_) => pieces::king::get_king_move_data(position, board),
        Piece::Knight(_) => pieces::knight::get_knight_move_data(position, board),
        Piece::Pawn(_) => pieces::pawn::get_pawn_move_data(position, board),
        Piece::Queen(_) => pieces::queen::get_queen_move_data(position, board),
        Piece::Rook(_) => pieces::rook::get_rook_move_data(position, board),
    }
    },
    None => todo!(),
  }
}