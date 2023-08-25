use std::collections::HashSet;
use std::collections::HashMap;

use crate::move_logger::MoveLogger;
use crate::{
  controller::Controller,
  view::View,
  board::Board,
  config::*,
  piece_move::PieceMove,
  position::Position, 
  pieces::{piece::*, self},
  move_data::MoveData
};

const VALID_PROMOTIONS: [&str; 4] = ["B", "N", "Q", "R"];

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
  pub white_long_castle: bool, // Whether white can still long castle
  pub white_short_castle: bool, // Whether white can still short castle
  pub black_long_castle: bool, // Whether black can still long castle
  pub black_short_castle: bool, // Whether black can still short castle
  pub in_check: bool, // Whether the current player's king is in check (updated for the next player after each move)
  pub valid_moves: HashMap<Position, Vec<Position>>, // A map of the current player's valid moves for each piece
  pub last_move: Option<PieceMove>
}

impl State {
  fn check_long_castle(&self) -> bool {
    return (self.white_turn && self.black_long_castle) || (!self.white_turn && self.white_long_castle);
  }
  fn check_short_castle(&self) -> bool {
    return (self.white_turn && self.black_short_castle) || (!self.white_turn && self.white_short_castle);
  }
}

pub struct Game<C: Controller, V: View> {
  controller: C,
  view: V,
  move_logger: MoveLogger,
  board: Board,
  state: State, // Holds the current state of the game
}

impl<C: Controller, V: View> Game<C, V> {
  /**
   * Initialises a standard chess game
   */
  pub fn new(controller: C, view: V, game_config: GameConfig) -> Self {
    // TODO: Add validation of the game config, checking for position boundaries single king per side
    let mut board = Board::new(&game_config.initial_board);
    Self {
      controller,
      view,
      move_logger: MoveLogger::new(board.get_current_board()),
      board,
      state: State {
        white_turn: game_config.white_turn,
        game_state: GameState::Active,
        white_long_castle: game_config.white_long_castle,
        white_short_castle: game_config.white_short_castle,
        black_long_castle: game_config.black_long_castle,
        black_short_castle: game_config.black_short_castle,
        in_check: false,
        valid_moves: HashMap::new(),
        last_move: None
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

    // Evaluate the starting board and update the game state with initial values
    self.update_game_state(!self.state.white_turn, &current_board);

    self.view.update_state(&current_board, self.state.clone());
    
    // Loop the turn based logic until there is an outcome for the game
    while let GameState::Active = self.state.game_state {
      let piece_move = self.controller.get_move(self.state.white_turn);

      // Validate the chosen move
      let is_valid = self.validate_move(&piece_move, &current_board);

      if is_valid {
        // Check move to update the castling options, if needed
        self.update_castling_options(&piece_move, &current_board);

        // Checks if the move made was a castling move and retrieves the rook move if it was
        let castle_move = self.get_castle_move(&piece_move, &current_board);

        // Checks if the move made was a en passant move and retrieves the taken piece if it was
        let en_passant_move = self.get_en_passant_move(&piece_move, &current_board);

        // The move is valid, make the move on the board and update the players with the current board state
        current_board = self.board.move_piece(&piece_move.start, &piece_move.end);

        // If this was a castling move then move the Rook piece as well
        if castle_move.is_some() {
          let c_move = castle_move.unwrap();
          current_board = self.board.move_piece(&c_move.start, &c_move.end);
        } else if en_passant_move.is_some() {
          // If this was an en passant move then remove the taken piece
          let ep_move = en_passant_move.unwrap();
          current_board = self.board.clear_position(&ep_move);
        } else if piece_move.promotion.is_some(){
          // If it is neither a castling move or en passant and the move has a supplied promotion piece
          let promoted_piece = self.get_promotion_piece(piece_move.promotion.as_ref().unwrap());
          if promoted_piece.is_some() {
            current_board = self.board.set_position(&piece_move.end, promoted_piece);
          } else {
            self.state.game_state = GameState::Error;
            continue;
          }
        }

        self.state.last_move = Some(piece_move.clone());

        // Evaluate the new board and update the game state
        self.update_game_state(self.state.white_turn, &current_board);

        // Update the move log
        self.move_logger.add_move(piece_move, &current_board, &self.state);

        // Swap the active player
        self.state.white_turn = !self.state.white_turn;
  
        // Update the player's views
        self.view.update_state(&current_board, self.state.clone());
      }
    }
  }

  /**
   * Validate the player move against the list of calculated valid moves in the game state. 
   * Returns true if the move can be made.
   */
  fn validate_move(&self, piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>) -> bool {
    if let Some(valid_positions) = self.state.valid_moves.get(&piece_move.start) {
      if valid_positions.contains(&piece_move.end) {
        // Validate promotion move
        // Check if the piece is on the furthest or nearest rank based on piece colour
        if (self.state.white_turn && piece_move.end.row == 7) || (!self.state.white_turn && piece_move.end.row == 0) {
          // Check if piece moved was a pawn
          match board[piece_move.start.row][piece_move.start.column].as_ref().unwrap() {
            Piece::Pawn(_) => {
              // Promotion not supplied when it should have been or provided promotion is invalid
              if piece_move.promotion.is_none() || !VALID_PROMOTIONS.contains(&piece_move.promotion.as_ref().unwrap().as_str()){
                return false;
              }
              return true; // Valid promotion move
            },
            _ => return piece_move.promotion.is_none() // Returns false if a promotion has been provided when it's not a promotion move
          }
        }

        // Returns false if a promotion has been provided when it's not a promotion move
        return piece_move.promotion.is_none(); 
      }
    }
    return false;
  }

  /**
   * Checks if a move involves the king or rooks and updates the castling options for the player if it does.
   * The supplied board should be the state of the board before the piece has been moved.
   */
  fn update_castling_options(&mut self, piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>) {
    match board[piece_move.start.row][piece_move.start.column].as_ref().unwrap() {
      Piece::Rook(_) => {
        if self.state.white_turn {
          if self.state.white_long_castle && piece_move.start.column == 0 {
            // White's turn, white has not yet castled, this moved rook is on the 1st File/column
            self.state.white_long_castle = false;
          } else if self.state.white_short_castle && piece_move.start.column == 7 {
            // White's turn, white has not yet castled, this moved rook is on the 8th File/column
            self.state.white_short_castle = false;
          }
        } else {
          if self.state.black_long_castle && piece_move.start.column == 0 {
            // Black's turn, black has not yet castled, this moved rook is on the 1st File/column
            self.state.black_long_castle = false;
          } else if self.state.black_short_castle && piece_move.start.column == 7 {
            // Black's turn, black has not yet castled, this moved rook is on the 8th File/column
            self.state.black_short_castle = false;
          }
        }
      },
      Piece::King(_) => {
        // As soon as king has move, regardless of if it was a castling move, castling is no longer available
        if self.state.white_turn {
          self.state.white_long_castle = false;
          self.state.white_short_castle = false;
        } else {
          self.state.black_long_castle = false;
          self.state.black_short_castle = false;
        }
      },
      _ => return
    }
  }

  /**
   * Checks the player's move to see if it was a castling move (king moving 2 spaces).
   * If it was a castling move then the move for the Rook is returned.
   * This assumes that the move has already been validated.
   */
  fn get_castle_move(&self, piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>) -> Option<PieceMove> {
    let column = piece_move.start.column;
    let target_column = piece_move.end.column;
    let row = piece_move.start.row;

    if let Piece::King(_) = board[row][column].as_ref().unwrap() {
      if target_column > column && target_column - column == 2 {
        // King-side castling move
        return Some(PieceMove {start: Position {row, column: 7}, end: Position {row, column: 5}, promotion: None});
      } else if column > target_column && column - target_column == 2 {
        // Queen-side castling move
        return Some(PieceMove {start: Position {row, column: 0}, end: Position {row, column: 3}, promotion: None});
      } else {
        // Not a castling move
        return None
      }
    }
    return None
  }

  /**
   * Checking if a given move was an en passant move and returning the position of the taken piece if it was
   */
  fn get_en_passant_move(&self, piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>) -> Option<Position> {
    // If the pawn changed File and the target position currently does not contain a piece then it can only be en passant
    if piece_move.end.column != piece_move.start.column && board[piece_move.end.row][piece_move.end.column].is_none() {
      return Some(Position {row: piece_move.start.row, column: piece_move.end.column});
    }
    None
  }

  /**
   * Returns a new Piece for the matching promotion id
   */
  fn get_promotion_piece(&self, promotion_id: &String) -> Option<Piece> {
    match promotion_id.as_str() {
      "B" => Some(Piece::Bishop(self.state.white_turn)),
      "N" => Some(Piece::Knight(self.state.white_turn)),
      "Q" => Some(Piece::Queen(self.state.white_turn)),
      "R" => Some(Piece::Rook(self.state.white_turn)),
      _ => None
    }
  }

  /**
   * Evaluates the current position of the board, searching for a check or a
   * checkmate on the opposing player. This function will return the state of 
   * check for the opposing player and the new game state.
   */
  fn update_game_state(&mut self, white_turn: bool, board: &Vec<Vec<Option<Piece>>>) {
    let mut opposing_king: Option<MoveData> = None;     // Move data for the opposing player's king
    let mut attacked_positions: Vec<Position> = vec![]; // List of all attacked positions for the current player
    let mut opposing_move_data: Vec<MoveData> = vec![]; // List of the move data for each piece of the opposing player
    let mut players_move_data: Vec<MoveData> = vec![];  // List of the move data for each piece of the current player
    let mut checking_pieces: Vec<MoveData> = vec![];    // List of the move data for the current player's piece which are checking the opposing king
    let mut defended_player_pieces: HashSet<Position> = HashSet::new(); // List of all the current player's pieces which are defended by another piece
    let mut pinned_positions: HashSet<Position> = HashSet::new();       // List of all opposing pieces pinned to the king

    let mut player_check = false;

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
            let move_data = self.get_move_data(&position, board);

            if (white_turn && !chess_piece.is_white()) || // White's turn, this is black piece or
                (!white_turn && chess_piece.is_white()) { // Black's turn, this is white piece
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
    let num_checking_pieces = checking_pieces.len();
    for mut move_data in opposing_move_data.as_mut_slice() {
      if pinned_positions.contains(&move_data.position) || num_checking_pieces > 1 {
        move_data.valid_moves = vec![];
      } else {
        opponent_can_move = true;

        // Determine a standard piece can block the check or capture the checker
        if num_checking_pieces == 1 {
          let checking_piece = &checking_pieces[0];
          let checking_path = checking_piece.checking_path.as_ref().unwrap(); // Checking pieces should always have Some(checking_path)
          
          let position = &checking_piece.position;

          let mut valid_moves = vec![];
          // Checking all the valid move positions by the opposing piece
          for attacked_position in &move_data.valid_moves {
            // If the checking piece can be captured 
            if attacked_position == position || checking_path.contains(attacked_position) {
              one_checker_valid_defend = true;
              valid_moves.push(attacked_position.to_owned());
            }
          }

          // Overwriting the piece's moves with the only valid move options
          move_data.valid_moves = valid_moves;
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
    let op_king = opposing_king.as_mut().unwrap();
    op_king.valid_moves = king_valid_moves;
    
    // Check whether the opponent's castling options are valid
    let king_position = &op_king.position;
    // Check long castle option
    if self.state.check_long_castle() {
      let long_castle_valid = pieces::king::is_king_long_castle_valid(king_position, board, &attacked_positions); // Whether the opposing king can long castle
      // If valid, add long castle move to king's valid moves
      if long_castle_valid {
        op_king.valid_moves.push(Position {row: king_position.row, column: 2});
      }
    }
    // Check short castle option
    if self.state.check_short_castle() {
      let short_castle_valid = pieces::king::is_king_short_castle_valid(king_position, board, &attacked_positions); // Whether the opposing king can short castle
      // If valid, add short castle move to king's valid moves
      if short_castle_valid {
        op_king.valid_moves.push(Position {row: king_position.row, column: 6});
      }
    }

    // Used in final game state checks
    let king_no_moves = op_king.valid_moves.is_empty();

    // Gather all valid moves for the opponent and set in game state
    // These moves will be used to validate the next input from the player
    let mut valid_moves = HashMap::new();
    // Add all the valid standard piece moves
    for move_data in opposing_move_data {
      if !move_data.valid_moves.is_empty() {
        // let mut v_moves = move_data.valid_moves;
        // if let Piece::Pawn(_) = board[move_data.position.row][move_data.position.column].as_ref().unwrap() {
        //   v_moves.extend(move_data.attacks);
        // }
        valid_moves.insert(move_data.position, move_data.valid_moves);
      }
    }
    // Add all the king valid moves
    let op_king = opposing_king.unwrap();
    valid_moves.insert(op_king.position, op_king.valid_moves);

    // Set the valid moves in the game state
    self.state.valid_moves = valid_moves;

    // If not check and no opposing piece has a valid move then stalemate
    if !player_check && !opponent_can_move {
      self.state.in_check = player_check;
      self.state.game_state = GameState::Stalemate;
      return;
    } else if player_check && king_no_moves && !one_checker_valid_defend {
      // Opposing player in checkmate, change state to end game
      if white_turn {
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

  /**
   * Get the relevant move data based on the Piece type in the given position
   */
  fn get_move_data(&self, position: &Position, board: &Vec<Vec<Option<Piece>>>) -> MoveData {
    match &board[position.row][position.column] {
      Some(piece) => {
        match piece {
          Piece::Bishop(_) => pieces::bishop::get_bishop_move_data(position, board),
          Piece::Knight(_) => pieces::knight::get_knight_move_data(position, board),
          Piece::Pawn(_) => pieces::pawn::get_pawn_move_data(position, board, &self.state.last_move),
          Piece::Queen(_) => pieces::queen::get_queen_move_data(position, board),
          Piece::Rook(_) => pieces::rook::get_rook_move_data(position, board),
          Piece::King(_) => pieces::king::get_king_move_data(position, board)
        }
      },
      None => todo!(),
    }
  }
}

