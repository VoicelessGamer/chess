use std::collections::HashSet;
use std::collections::HashMap;

use crate::{
  board::Board,
  config::*,
  model::{CastlingState, GameState, GameStateResult, PieceMove, PlayerState, Position, State},
  pieces::{piece::*, self},
  move_data::MoveData
};

pub const VALID_PROMOTIONS: [&str; 4] = ["B", "N", "Q", "R"];

struct StateChangeResult {
  pub state: State,
  pub white_check: bool,
  pub black_check: bool,
  pub white_moves: HashMap<Position, Vec<Position>>,
  pub black_moves: HashMap<Position, Vec<Position>>
}

impl Default for StateChangeResult {
  fn default() -> Self {
    Self { 
      state: State::Active, 
      white_check: false,
      black_check: false,
      white_moves: HashMap::new(),
      black_moves: HashMap::new()
    }
  }
}

pub struct Game {
  board: Board,
  pub game_state: GameState
}

struct PositionalData {
  pub white_moves: Vec<MoveData>, // List of the move data for each white piece on the board (includes king)
  pub black_moves: Vec<MoveData>, // List of the move data for each black piece on the board (includes king)
  pub white_king_index: i32, // Index to the move data for the white king in the white_moves vec
  pub black_king_index: i32 // Index to the move data for the black king in the black_moves vec
}

impl Game {
  /**
   * Initialises a chess game from the provided config.
   */
  pub fn new(game_config: GameConfig) -> Self {
    let board = Board::new(&game_config.board);

    let white_castling_state = CastlingState {
      long_castle: game_config.white_castling.long_castle,
      short_castle: game_config.white_castling.short_castle
    };
    let black_castling_state = CastlingState {
      long_castle: game_config.black_castling.long_castle,
      short_castle: game_config.black_castling.short_castle
    };

    let initial_state = match game_config.white_turn {
        true => get_state_change(board.board(), game_config.white_turn, &white_castling_state, &None),
        false => get_state_change(board.board(), game_config.white_turn, &black_castling_state, &None)
    };

    Self {
      board,
      game_state: GameState {
        state: initial_state.state,
        white_turn: game_config.white_turn,
        white_state: PlayerState {
          in_check: initial_state.white_check,
          castling_state: white_castling_state,
          valid_moves: initial_state.white_moves,
          last_move: None
        },
        black_state: PlayerState {
          in_check: initial_state.black_check,
          castling_state: black_castling_state,
          valid_moves: initial_state.black_moves,
          last_move: None
        }
      }
    }
  }

  /**
   * Returns the current game state
   */
  pub fn get_game_state(&mut self) -> GameStateResult {
    return GameStateResult {board: self.board.copy_board(), game_state: self.game_state.clone()};
  }

  /**
   * Given a piece move, validates the move, updates the board and the game's state to reflect the changes
   */
  pub fn process_move(&mut self, piece_move: PieceMove) -> Result<GameStateResult, String> {
    // self.game_state.state = State::Draw;
    if self.game_state.state != State::Active {
      return Err(format!("Game state is {:?}. Cannot perform any further actions.", self.game_state.state));
    } 

    let mut current_board = self.board.copy_board();


    if !self.validate_move(&piece_move) {
      return Err("Selected move validation failed.".to_string());
    }

    // Check move to update the castling options, if needed
    self.update_castling_options(&piece_move, &current_board);

    // Checks if the move made was a castling move and retrieves the rook move if it was
    let castle_move = pieces::king::get_castle_move(&piece_move, &current_board);

    let piece_type = current_board[piece_move.start.row][piece_move.start.column].as_ref().unwrap().clone();
    
    // Checks if the move made was a en passant move and retrieves the taken piece if it was
    let en_passant_move = match piece_type {
      Piece::Pawn(_) => pieces::pawn::get_en_passant_move(&piece_move, &current_board),
      _ => None
    };

    // The move is valid, make the move on the board and update the players with the current board state
    let move_result = self.board.move_piece(&piece_move.start, &piece_move.end);
    if move_result.is_err() {
      return Err(move_result.unwrap_err().to_string());
    }
    current_board = move_result.unwrap();

    // If this was a castling move then move the Rook piece as well
    if castle_move.is_some() {
      let c_move = castle_move.unwrap();
      let move_result = self.board.move_piece(&c_move.start, &c_move.end);
      if move_result.is_err() {
        return Err(move_result.unwrap_err().to_string());
      }
      current_board = move_result.unwrap();
    } else if en_passant_move.is_some() {
      // If this was an en passant move then remove the taken piece
      let ep_move = en_passant_move.unwrap();
      current_board = self.board.set_position(&ep_move, None);
    } else if piece_move.promotion.is_some() && piece_type == Piece::Pawn(self.game_state.white_turn){
      // If it is neither a castling move or en passant and the move has a supplied promotion piece
      let promoted_piece = pieces::piece::get_promotion_piece(piece_move.promotion.as_ref().unwrap(), self.game_state.white_turn);
      if promoted_piece.is_some() {
        current_board = self.board.set_position(&piece_move.end, promoted_piece);
      } else {
        self.game_state.state = State::Error;
        return Err("Missing promotion choice.".to_string());
      }
    }

    if self.game_state.white_turn {
      self.game_state.white_state.last_move = Some(piece_move.clone());
    } else {
      self.game_state.black_state.last_move = Some(piece_move.clone());
    }

    // Swap the active player
    self.game_state.white_turn = !self.game_state.white_turn;

    // Evaluate the new board and update the game state
    // let state_change = 
    self.update_game_state(match self.game_state.white_turn {
        true => get_state_change(
          &current_board, true, 
          &self.game_state.white_state.castling_state, 
          &self.game_state.black_state.last_move
        ),
        false => get_state_change(
          &current_board, 
          false, 
          &self.game_state.black_state.castling_state, 
          &self.game_state.white_state.last_move
        ),
    });

    return Ok(GameStateResult {board: current_board, game_state: self.game_state.clone()});
  }

  /**
   * Checks the provided piece movement is valid based on previously calculated valid options for the current active player.
   * Returns true if the movement is valid.
   */
  fn validate_move(&self, piece_move: &PieceMove) -> bool {

    // Check move is valid using the list of valid moves calculated on the previous turn
    let valid_moves = match self.game_state.white_turn {
      true => &self.game_state.white_state.valid_moves,
      false => &self.game_state.black_state.valid_moves
    };

    let valid_positions_result = valid_moves.get(&piece_move.start);

    if valid_positions_result.is_none() {
      return false;
    }

    if !valid_positions_result.unwrap().contains(&piece_move.end) {
      return false;
    }

    // Validate promotion move
    match self.board.board()[piece_move.start.row][piece_move.start.column].as_ref().unwrap() {
      Piece::Pawn(_) => {// Check if piece moved was a pawn
        // Check if the piece is on the furthest or nearest rank based on piece colour
        if (self.game_state.white_turn && piece_move.end.row == 7) || (!self.game_state.white_turn && piece_move.end.row == 0) {
          // Promotion not supplied when it should have been or provided promotion is invalid
          return piece_move.promotion.is_some() && VALID_PROMOTIONS.contains(&piece_move.promotion.as_ref().unwrap().as_str());
        }

        return piece_move.promotion.is_none() // Returns false if a promotion has been provided when it's not a promotion move
      },
      _ => return piece_move.promotion.is_none() // Returns false if a promotion has been provided when it's not a promotion move
    }
  }

  /**
   * Checks if a move involves the king or rooks and updates the castling options for the player if it does.
   * The supplied board should be the state of the board before the piece has been moved.
   */
  fn update_castling_options(&mut self, piece_move: &PieceMove, current_board: &Vec<Vec<Option<Piece>>>) {
    match current_board[piece_move.start.row][piece_move.start.column].as_ref().unwrap() {
      Piece::Rook(_) => {
        if self.game_state.white_turn {
          if self.game_state.white_state.castling_state.long_castle && piece_move.start.column == 0 {
            // White's turn, white has not yet castled, this moved rook is on the 1st File/column
            self.game_state.white_state.castling_state.long_castle = false;
          } else if self.game_state.white_state.castling_state.short_castle && piece_move.start.column == 7 {
            // White's turn, white has not yet castled, this moved rook is on the 8th File/column
            self.game_state.white_state.castling_state.short_castle = false;
          }
        } else {
          if self.game_state.black_state.castling_state.long_castle && piece_move.start.column == 0 {
            // Black's turn, black has not yet castled, this moved rook is on the 1st File/column
            self.game_state.black_state.castling_state.long_castle = false;
          } else if self.game_state.black_state.castling_state.short_castle && piece_move.start.column == 7 {
            // Black's turn, black has not yet castled, this moved rook is on the 8th File/column
            self.game_state.black_state.castling_state.short_castle = false;
          }
        }
      },
      Piece::King(_) => {
        // As soon as king has moved, regardless of if it was a castling move, castling is no longer available
        if self.game_state.white_turn {
          self.game_state.white_state.castling_state.long_castle = false;
          self.game_state.white_state.castling_state.short_castle = false;
        } else {
          self.game_state.black_state.castling_state.long_castle = false;
          self.game_state.black_state.castling_state.short_castle = false;
        }
      },
      _ => return
    }
  }

  /**
   * Updates the current state with the state changes. 
   */
  fn update_game_state(&mut self, state_change: StateChangeResult) {
    self.game_state.state = state_change.state;
    self.game_state.white_state.in_check = state_change.white_check;
    self.game_state.white_state.valid_moves = state_change.white_moves;
    self.game_state.black_state.in_check = state_change.black_check;
    self.game_state.black_state.valid_moves = state_change.black_moves;
  }

}


/**
 * Evaluates the current position of the board, searching for a check or a
 * checkmate on the current player. This function will update the state of 
 * check for the current player and the new game state.
 */
fn get_state_change(board: &Vec<Vec<Option<Piece>>>, white_turn: bool, castling_state: &CastlingState, opponent_last_move: &Option<PieceMove>) -> StateChangeResult {
  let mut state_change_result = StateChangeResult::default();

  let collected_data = collect_positional_data(board, white_turn, opponent_last_move);
  if collected_data.is_none() {
    state_change_result.state = State::Error;
    return state_change_result;
  }

  let mut positional_data: PositionalData = collected_data.unwrap();

  // If either king was not found then there has been an error in gameplay/logic, cannot continue
  if positional_data.white_king_index == -1 || positional_data.black_king_index == -1 {
    state_change_result.state = State::Error;
    return state_change_result;
  }

  match white_turn {
    true => 
      update_king_valid_moves(&mut positional_data.white_moves[positional_data.white_king_index as usize], &positional_data.black_moves),
    false => 
      update_king_valid_moves(&mut positional_data.black_moves[positional_data.black_king_index as usize], &positional_data.white_moves),
  }

  // These moves will be used to validate the next input from the player
  let mut valid_moves = HashMap::new();

  // Determine state of check for the current players' king
  if is_checked(white_turn, &positional_data) {
    // Update the check flag for the current player
    match white_turn {
      true => state_change_result.white_check = true,
      false => state_change_result.black_check = true,
    };

    // Check for game win scenario
    // Collect all the valid moves for each of the current players' pieces
    valid_moves = match white_turn {
      true => {
        collect_valid_moves(
          positional_data.white_moves.clone(), 
          positional_data.white_king_index as usize,
          castling_state,
          positional_data.black_moves.clone(), 
          board
        )
      },
      false => {
        collect_valid_moves(
          positional_data.black_moves.clone(), 
          positional_data.black_king_index as usize,
          castling_state,
          positional_data.white_moves.clone(), 
          board
        )
      }
    };

    if valid_moves.is_empty() {
      state_change_result.state = match white_turn {
        true => State::BlackWin,
        false => State::WhiteWin
      };
    }
  } else {
    // Not in check or checkmate position
    // Collect all the valid moves for each of the current players' pieces
    let players_move_data: &Vec<MoveData>;
    let pinned_positions: HashMap<Position, Position>;
    match white_turn {
      true => {
        players_move_data = &positional_data.white_moves;
        pinned_positions = get_pinned_position_map(&positional_data.black_moves);
      }
      false => {
        players_move_data = &positional_data.black_moves;
        pinned_positions = get_pinned_position_map(&positional_data.white_moves);
      }
    };

    for move_data in players_move_data {
      if move_data.valid_moves.is_empty() {
        continue;          
      }

      // Remove invalid moves due to any pins
      if pinned_positions.contains_key(&move_data.position) {
        valid_moves.insert(move_data.position.clone(), adjust_pinned_valid_moves(&move_data, pinned_positions.get(&move_data.position).unwrap(), board));
      } else {
        valid_moves.insert(move_data.position.clone(), move_data.valid_moves.clone()); 
      }
    }

    // If valid_moves is empty -> not in check or checkmate and has no valid moves, so stalemate
    // or 
    // If both players have insufficent pieces to force a checkmate then it's a draw
    if valid_moves.is_empty() || 
        (!has_sufficient_material(&positional_data.white_moves, &board) && !has_sufficient_material(&positional_data.black_moves, &board)) {
      state_change_result.state = State::Draw;
    }
  }

  //Update the valid moves list for the current player
  match white_turn {
    true => state_change_result.white_moves = valid_moves,
    false => state_change_result.black_moves = valid_moves,
  };

  return state_change_result;
}

/**
 * Collects the positional and movement data for all pieces on the board and returns the data in a PositionalData struct.
 * NOTE: This gathers the potential positional movement data for each piece on the board based on the movement pattern for the individual piece.
 * The does not take into account any other piece, i.e. whether itself is pinned to the king, or if the king piece cannot move due to attacked positions.
 */
fn collect_positional_data(board: &Vec<Vec<Option<Piece>>>, white_turn: bool, opponent_last_move: &Option<PieceMove>) -> Option<PositionalData> {
  let mut white_moves: Vec<MoveData> = vec![]; // List of the move data for each white piece on the board (includes king)
  let mut black_moves: Vec<MoveData> = vec![]; // List of the move data for each black piece on the board (includes king)
  let mut white_king_index: i32 = -1; // Index to the move data for the white king in the white_moves vec
  let mut black_king_index: i32 = -1; // Index to the move data for the black king in the black_moves vec
  
  // Retrieve all the positional and move information for each piece on the board to determine check state
  for i in 0..board.len() {
    let row = &board[i];
    for j in 0..row.len() {
      let piece = &row[j];
      match piece {
        None => continue,
        Some(chess_piece) => {
          let position = Position {row: i, column: j};

          
          if chess_piece.is_white() {
            let move_data: MoveData;
            if white_turn {
              move_data = pieces::get_move_data(&position, board, opponent_last_move)?;
            } else {
              // Passing None for the last_move field as it's not needed when calculating positional data for the opposing side's pieces
              // Passing the last move here would cause an issue with the en passant calculations due to the piece no longe being on the board
              move_data = pieces::get_move_data(&position, board, &None)?;
            }
            white_moves.push(move_data);

            if chess_piece.is_king() {
              white_king_index = (white_moves.len() - 1) as i32;
            }
          } else {
            let move_data: MoveData;
            if white_turn {
              // Passing None for the last_move field as it's not needed when calculating positional data for the opposing side's pieces
              // Passing the last move here would cause an issue with the en passant calculations due to the piece no longe being on the board
              move_data = pieces::get_move_data(&position, board, &None)?;
            } else {
              move_data = pieces::get_move_data(&position, board, opponent_last_move)?;
            }
            black_moves.push(move_data);

            if chess_piece.is_king() {
              black_king_index = (black_moves.len() - 1) as i32;
            }
          }
        }
      }
    }
  }

  return Some(PositionalData {
    white_moves,
    black_moves,
    white_king_index,
    black_king_index
  })
}

/**
 * Returns true if the current players' king is currently under attack by any opposing piece.
 */
fn is_checked(white_turn: bool, positional_data: &PositionalData) -> bool {
  if white_turn {
    return king_is_attacked(&positional_data.black_moves);
  } else {
    return king_is_attacked(&positional_data.white_moves);
  }
}

/**
 * Updates the valid move list for a given king to remove all positions that are attacked by the opposing players' pieces.
 */
fn update_king_valid_moves(king_data: &mut MoveData, opposing_pieces: &Vec<MoveData>) {
  let mut attacked_positions: Vec<Position> = vec![]; // List of all attacked positions by the opposing player
  let mut defended_positions: Vec<Position> = vec![]; // List of all defended positions by the opposing player

  // Retrieve all the relevant data needed to determine a checkmate scenario
  for move_data in opposing_pieces {
    let cloned_data = move_data.clone();
    attacked_positions.extend(cloned_data.attacks);
    defended_positions.extend(cloned_data.defends);
  }

  let mut updated_moves: Vec<Position> = vec![];
  for position in &king_data.valid_moves {
    if !attacked_positions.contains(position) && !defended_positions.contains(position) {
      updated_moves.push(position.clone());
    }
  }

  king_data.valid_moves = updated_moves;
}

/**
 * Returns true if any opposing piece has a checking path towards the current players' king.
 */
fn king_is_attacked(opposing_pieces: &Vec<MoveData>) -> bool {
  for move_data in opposing_pieces {
    if move_data.checking_path.is_some() {
      return true;
    }
  }

  return false
}

/**
 * Evaluates the current position of the board, and collects the valid moves for the current defending player.
 */
fn collect_valid_moves(mut defending_pieces: Vec<MoveData>, defending_king_index: usize, 
    defending_castling_state: &CastlingState, attacking_pieces: Vec<MoveData>, board: &Vec<Vec<Option<Piece>>>) -> HashMap<Position, Vec<Position>>  {
  // TODO: Could do with adding some tests for this function, but they would be a lot of work
  let mut attacked_positions: Vec<Position> = vec![]; // List of all attacked positions by the opposing player
  let mut opposing_move_data: Vec<MoveData> = vec![]; // List of the move data for each piece of the opposing player
  let mut checking_pieces: Vec<MoveData> = vec![];    // List of the move data for the opposing player's pieces which are checking the current player's king
  let mut defended_player_pieces: HashSet<Position> = HashSet::new(); // List of all the opposing player's pieces which are defended by another piece
  let mut pinned_positions: HashMap<Position, Position> = HashMap::new();

  // Retrieve all the relevant data needed to determine a checkmate scenario
  for move_data in attacking_pieces {
    let cloned_data = move_data.clone();
    attacked_positions.extend(cloned_data.attacks);
    defended_player_pieces.extend(cloned_data.defends);
    opposing_move_data.push(move_data.clone());
    for position in &move_data.pins {
      // NOTE: Should only be possible to be pinned by a single piece
      pinned_positions.insert(position.clone(), move_data.position.clone());
    }
    if move_data.checking_path.is_some() {
      checking_pieces.push(move_data);
    }
  }

  // Calculate the valid moves for all current players' piece
  let num_checking_pieces = checking_pieces.len();
  // let mut one_checker_valid_defend: bool = false;
  for move_data in defending_pieces.as_mut_slice() {
    if num_checking_pieces > 1 {
      // Pinned piece cannot move
      move_data.valid_moves = vec![];
    } else {
      let current_valid_moves: Vec<Position>;
      if pinned_positions.contains_key(&move_data.position) {
        current_valid_moves = adjust_pinned_valid_moves(&move_data, pinned_positions.get(&move_data.position).unwrap(), board);
      } else {
        current_valid_moves = move_data.valid_moves.clone();
      }

      // Determine a standard piece can block the check or capture the checker
      if num_checking_pieces != 1 { // Can only be 1 or 0 at this point
        continue;
      }

      let checking_piece = &checking_pieces[0];
      let checking_path = checking_piece.checking_path.as_ref().unwrap(); // Checking pieces should always have Some(checking_path)    
      let position = &checking_piece.position;

      let mut valid_moves = vec![];
      // Checking all the valid move positions by the opposing piece
      for attacked_position in &current_valid_moves{
        // If the checking piece can be captured 
        if attacked_position == position || checking_path.contains(attacked_position) {
          // one_checker_valid_defend = true;
          valid_moves.push(attacked_position.to_owned());
        }
      }

      // Overwriting the piece's moves with the only valid move options
      move_data.valid_moves = valid_moves;
    }
  }

  let current_king = &mut defending_pieces[defending_king_index];

  // For the current king remove any moves that are not valid based on the opposing players' pieces
  let mut king_valid_moves = vec![];
  for position in &current_king.valid_moves {
    // Check to make sure the position is not attacked by an opposing piece, and isn't a defended opposing piece position
    if !attacked_positions.contains(position) && !defended_player_pieces.contains(position) {
      // Adding the valid moves (non defended positions) to a separate vec
      king_valid_moves.push(position.clone());
    }
  }

  // Overwriting the kings valid moves
  current_king.valid_moves = king_valid_moves;
  
  // Check whether the player's castling options are valid
  let king_position = &current_king.position;

  // Check long castle option
  if defending_castling_state.long_castle {
    let long_castle_valid = pieces::king::is_king_long_castle_valid(king_position, board, &attacked_positions); // Whether the opposing king can long castle
    // If valid, add long castle move to king's valid moves
    if long_castle_valid {
      current_king.valid_moves.push(Position {row: king_position.row, column: 2});
    }
  }

  // Check short castle option
  if defending_castling_state.short_castle {
    let short_castle_valid = pieces::king::is_king_short_castle_valid(king_position, board, &attacked_positions); // Whether the opposing king can short castle
    // If valid, add short castle move to king's valid moves
    if short_castle_valid {
      current_king.valid_moves.push(Position {row: king_position.row, column: 6});
    }
  }

  // Gather all valid moves for the player and set in game state
  // These moves will be used to validate the next input from the player
  let mut valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();

  // Add all the valid standard piece moves
  for move_data in defending_pieces {
    if !move_data.valid_moves.is_empty() {
      valid_moves.insert(move_data.position, move_data.valid_moves);
    }
  }

  return valid_moves;
}

/**
 * Iterates the provided move data and returns a map of the pinned piece position to the attacking piece position.
 */
fn get_pinned_position_map(attacking_pieces: &Vec<MoveData>) -> HashMap<Position, Position> {
  let mut pinned_positions: HashMap<Position, Position> = HashMap::new();

  // Retrieve the pinned piece data (pinned piece position -> attacking piece position)
  for move_data in attacking_pieces {
    for position in &move_data.pins {
      // NOTE: Should only be possible to be pinned by a single piece
      pinned_positions.insert(position.clone(), move_data.position.clone());
    }
  }

  return pinned_positions;
}

/**
 * Adjusts the provided move_data vec by removing all positions that are not possible due to the opposing piece's pin. 
 */
fn adjust_pinned_valid_moves(move_data: &MoveData, attacking_piece_position: &Position, board: &Vec<Vec<Option<Piece>>>) -> Vec<Position> {
  let mut adjusted_move_data: Vec<Position> = vec![];
  let mut sim_board = board.clone();
  let mut current_position: Position = move_data.position.clone();

  for position in &move_data.valid_moves {
    // Move piece to next valid position
    let chess_piece = sim_board[current_position.row][current_position.column].take();
    sim_board[position.row][position.column] = chess_piece;
    current_position = position.clone();

    let collected_move_data = pieces::get_move_data(attacking_piece_position, &sim_board, &None);
    if collected_move_data.is_none() {
      // Shouldn't be possible to be none if all other checks are done before this function call, but this way it won't panic if it is
      continue;
    }
    
    let attacker_move_data: MoveData = collected_move_data.unwrap();
    if attacker_move_data.checking_path.is_none() {
      adjusted_move_data.push(position.clone());
    }
  }

  return adjusted_move_data;
}

/**
 * Checks for any special stalemate edge cases, based on remaining pieces. Returns true if the player has enough of the correct pieces to force a checkmate.
 */
fn has_sufficient_material(moves: &Vec<MoveData>, board: &Vec<Vec<Option<Piece>>>) -> bool {
  let mut num_knights: i32 = 0;
  let mut has_dark_bishop: bool = false;
  let mut has_light_bishop: bool = false;

  for move_data in moves {
    let piece = board[move_data.position.row][move_data.position.column].as_ref();
    if piece.is_none() {
      continue;
    }
    match piece.unwrap() {
      Piece::Queen(_) | Piece::Pawn(_) | Piece::Rook(_) => return true,
      Piece::Bishop(_) => {
        if (move_data.position.row + move_data.position.column) % 2 == 0 {
          has_dark_bishop = true;
        } else {
          has_light_bishop = true;
        }
      },
      Piece::Knight(_) => num_knights += 1,
      Piece::King(_) => continue,
    }
  }

  if num_knights > 2 ||
      (has_dark_bishop && has_light_bishop) {
     return true;
  }

  return false;
}

#[cfg(test)]
mod game_tests {
  use std::collections::HashMap;

  use crate::{config::{BoardConfig, CastlingConfig, GameConfig, PieceConfig}, game::State, model::{PieceMove, Position}, pieces::piece::Piece};

  use super::Game;

  /**
   * Tests the game construction with an invalid game configuration.
   */
  #[test]
  fn invalid_game_initialisation() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };

    let mut game = Game::new(game_config);

    assert!(game.get_game_state().game_state.state == State::Error);
  }

  /**
   * Tests the process_move function returns an error when the game is in an errored state.
   */
  #[test]
  fn move_attempt_on_errored_game() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 5}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);
    game.game_state.state = State::Error;

    let piece_move = PieceMove{start: Position {row: 5, column: 2}, end: Position {row: 6, column: 2}, promotion: None};

    assert!(game.process_move(piece_move).is_err());
  }

  /**
   * Tests the process_move function returns an error when the provided move is invalid.
   */
  #[test]
  fn invalid_move_attempt() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 0, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 5, column: 2}, vec![Position{row: 6, column: 2}]);

    game.game_state.white_state.valid_moves = white_valid_moves;

    let piece_move = PieceMove{start: Position {row: 5, column: 2}, end: Position {row: 6, column: 1}, promotion: None};

    assert!(game.process_move(piece_move).is_err());
  }

  /**
   * Tests the process_move function with a long castle move, ensuring the board is updated correctly.
   */
  #[test]
  fn long_castle_move_performed() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 0, column: 4}, vec![Position{row: 0, column: 2}]); // Castling move

    game.game_state.white_state.valid_moves = white_valid_moves;

    let piece_move = PieceMove{start: Position {row: 0, column: 4}, end: Position {row: 0, column: 2}, promotion: None};

    let result = game.process_move(piece_move);

    assert!(result.is_ok());

    let game_result = result.unwrap();

    assert!(game_result.board[0][2].is_some());
    assert!(game_result.board[0][3].is_some());
    
    assert!(game_result.board[0][2].as_ref().unwrap() == &Piece::King(true));
    assert!(game_result.board[0][3].as_ref().unwrap() == &Piece::Rook(true));
  }

  /**
   * Tests the process_move function with a short castle move, ensuring the board is updated correctly.
   */
  #[test]
  fn short_castle_move_performed() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 0, column: 4}, vec![Position{row: 0, column: 6}]); // Castling move

    game.game_state.white_state.valid_moves = white_valid_moves;

    let piece_move = PieceMove{start: Position {row: 0, column: 4}, end: Position {row: 0, column: 6}, promotion: None};

    let result = game.process_move(piece_move);

    assert!(result.is_ok());

    let game_result = result.unwrap();

    assert!(game_result.board[0][6].is_some());
    assert!(game_result.board[0][5].is_some());
    
    assert!(game_result.board[0][6].as_ref().unwrap() == &Piece::King(true));
    assert!(game_result.board[0][5].as_ref().unwrap() == &Piece::Rook(true));
  }

  /**
   * Tests the process_move function with a long castle move, ensuring the board is updated correctly.
   */
  #[test]
  fn non_pawn_promotion_ignored() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 0, column: 7}, vec![Position{row: 7, column: 7}]);

    game.game_state.white_state.valid_moves = white_valid_moves;

    // Invalid promotion field added
    let piece_move = PieceMove{start: Position {row: 0, column: 7}, end: Position {row: 7, column: 7}, promotion: Some("Q".to_string())};

    let result = game.process_move(piece_move);

    assert!(result.is_err());
  }

  /**
   * Tests the process_move function with an en passant move, ensuring the board is updated correctly.
   */
  #[test]
  fn en_passant_performed() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 4},
          PieceConfig {piece: String::from("pawn"), white: false, column: 0, row: 4},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 4, column: 1}, vec![Position{row: 5, column: 1}, Position{row: 5, column: 0}]);

    game.game_state.white_state.valid_moves = white_valid_moves;
    game.game_state.black_state.last_move = Some(PieceMove{start: Position{row: 6, column: 0}, end: Position{row: 4, column: 0}, promotion: None});

    let piece_move = PieceMove{start: Position {row: 4, column: 1}, end: Position {row: 5, column: 0}, promotion: None};

    let result = game.process_move(piece_move);

    assert!(result.is_ok());

    let game_result = result.unwrap();

    assert!(game_result.board[5][0].is_some());
    assert!(game_result.board[4][0].is_none());
    
    assert!(game_result.board[5][0].as_ref().unwrap() == &Piece::Pawn(true));
  }

  /**
   * Tests the process_move function to ensure it rotates the current active player after the move has been performed.
   */
  #[test]
  fn turn_alternation() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 3, row: 2},
          PieceConfig {piece: String::from("pawn"), white: false, column: 0, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    assert!(game.game_state.white_turn == true);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 2, column: 3}, vec![Position{row: 3, column: 3}]);

    game.game_state.white_state.valid_moves = white_valid_moves;

    let mut piece_move = PieceMove{start: Position {row: 2, column: 3}, end: Position {row: 3, column: 3}, promotion: None};

    let result = game.process_move(piece_move);

    assert!(result.is_ok());
    assert!(game.game_state.white_turn == false);

    piece_move = PieceMove{start: Position {row: 5, column: 0}, end: Position {row: 4, column: 0}, promotion: None};

    let result = game.process_move(piece_move);

    assert!(result.is_ok());
    assert!(game.game_state.white_turn == true);
  }

  /**
   * Tests the validate_move function returns true when providing a possible move for white.
   */
  #[test]
  fn white_valid_move_validation() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 0, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 5, column: 2}, vec![Position{row: 6, column: 2}]);

    game.game_state.white_state.valid_moves = white_valid_moves;

    let piece_move = PieceMove{start: Position {row: 5, column: 2}, end: Position {row: 6, column: 2}, promotion: None};

    assert!(game.validate_move(&piece_move));
  }

  /**
   * Tests the validate_move function returns true when providing a possible move for black.
   */
  #[test]
  fn black_valid_move_validation() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 0, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: false
    };
  
    let mut game = Game::new(game_config);

    let mut black_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    black_valid_moves.insert(Position{row: 7, column: 0}, vec![Position{row: 7, column: 1}, Position{row: 6, column: 0}]);

    game.game_state.black_state.valid_moves = black_valid_moves;

    let piece_move = PieceMove{start: Position {row: 7, column: 0}, end: Position {row: 6, column: 0}, promotion: None};

    assert!(game.validate_move(&piece_move));
  }

  /**
   * Tests the validate_move function returns true when providing a move for a position that doesn't contain a piece.
   */
  #[test]
  fn invalid_start_position_move_validation() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 0, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 5, column: 2}, vec![Position{row: 6, column: 2}]);

    game.game_state.white_state.valid_moves = white_valid_moves;

    let piece_move = PieceMove{start: Position {row: 4, column: 4}, end: Position {row: 5, column: 1}, promotion: None};

    assert!(!game.validate_move(&piece_move));
  }

  /**
   * Tests the validate_move function returns true when providing an impossible move for a valid piece.
   */
  #[test]
  fn invalid_end_position_move_validation() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 2, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 0, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      black_castling: CastlingConfig {
        long_castle: false,
        short_castle: false
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let mut white_valid_moves: HashMap<Position, Vec<Position>> = HashMap::new();
    white_valid_moves.insert(Position{row: 5, column: 2}, vec![Position{row: 6, column: 2}]);

    game.game_state.white_state.valid_moves = white_valid_moves;

    let piece_move = PieceMove{start: Position {row: 5, column: 2}, end: Position {row: 6, column: 1}, promotion: None};

    assert!(!game.validate_move(&piece_move));
  }

  /**
   * Tests the update_castling_options function correctly updates the castling options after the queen-side rook is moved
   */
  #[test]
  fn long_castling_option_rook_move_white() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let current_board = game.board.copy_board();

    game.update_castling_options(&PieceMove { start: Position{ row: 0, column: 0 }, end: Position{ row: 1, column: 0 }, promotion: None}, &current_board);

    assert!(!game.game_state.white_state.castling_state.long_castle);
    assert!(game.game_state.white_state.castling_state.short_castle); // Short castle should still be available
  }

  /**
   * Tests the update_castling_options function correctly updates the castling options after the queen-side rook is moved
   */
  #[test]
  fn long_castling_option_rook_move_black() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 7},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 7},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: false
    };
  
    let mut game = Game::new(game_config);

    let current_board = game.board.copy_board();

    game.update_castling_options(&PieceMove { start: Position{ row: 7, column: 0 }, end: Position{ row: 6, column: 0 }, promotion: None}, &current_board);

    assert!(!game.game_state.black_state.castling_state.long_castle);
    assert!(game.game_state.black_state.castling_state.short_castle); // Short castle should still be available
  }

  /**
   * Tests the update_castling_options function correctly updates the castling options after the king-side rook is moved
   */
  #[test]
  fn short_castling_option_rook_move_white() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let current_board = game.board.copy_board();

    game.update_castling_options(&PieceMove { start: Position{ row: 0, column: 7 }, end: Position{ row: 1, column: 7 }, promotion: None}, &current_board);

    assert!(game.game_state.white_state.castling_state.long_castle); // Long castle should still be available
    assert!(!game.game_state.white_state.castling_state.short_castle); 
  }

  /**
   * Tests the update_castling_options function correctly updates the castling options after the king-side rook is moved
   */
  #[test]
  fn short_castling_option_rook_move_black() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 7},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 7},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: false
    };
  
    let mut game = Game::new(game_config);

    let current_board = game.board.copy_board();

    game.update_castling_options(&PieceMove { start: Position{ row: 7, column: 7 }, end: Position{ row: 6, column: 7 }, promotion: None}, &current_board);

    assert!(game.game_state.black_state.castling_state.long_castle); // Long castle should still be available
    assert!(!game.game_state.black_state.castling_state.short_castle); 
  }

  /**
   * Tests the update_castling_options function correctly updates the castling options after the king is moved
   */
  #[test]
  fn castling_options_king_move_white() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };
  
    let mut game = Game::new(game_config);

    let current_board = game.board.copy_board();

    game.update_castling_options(&PieceMove { start: Position{ row: 0, column: 4 }, end: Position{ row: 1, column: 4 }, promotion: None}, &current_board);

    assert!(!game.game_state.white_state.castling_state.long_castle);
    assert!(!game.game_state.white_state.castling_state.short_castle); 
  }

  /**
   * Tests the update_castling_options function correctly updates the castling options after the king is moved
   */
  #[test]
  fn castling_options_king_move_black() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 7},
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 7},
          PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: false
    };
  
    let mut game = Game::new(game_config);

    let current_board = game.board.copy_board();

    game.update_castling_options(&PieceMove { start: Position{ row: 7, column: 4 }, end: Position{ row: 6, column: 4 }, promotion: None}, &current_board);

    assert!(!game.game_state.black_state.castling_state.long_castle);
    assert!(!game.game_state.black_state.castling_state.short_castle); 
  }

  /**
   * Tests the validate_move_selection function correctly identifies the provided move is valid pawn promotion and returns true.
   */
  #[test]
  fn valid_promotion_move_selection() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 6},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let mut game = Game::new(game_config);

    let mut white_moves = HashMap::new();
    white_moves.insert(Position{row: 6, column: 0}, vec![Position{row: 7, column: 0}]);

    game.game_state.white_state.valid_moves = white_moves;

    let move_result = game.validate_move(
      &PieceMove {start: Position{row: 6, column: 0}, end: Position{row: 7, column: 0}, promotion: Some("Q".to_string())}
    );

    assert!(move_result);
  }

  /**
   * Tests the validate_move_selection function with a promotion provided on a pawn move that doesn't reach the required rank. Should return false.
   */
  #[test]
  fn invalid_promotion_move_selection() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let mut game = Game::new(game_config);

    let mut white_moves = HashMap::new();
    white_moves.insert(Position{row: 5, column: 0}, vec![Position{row: 6, column: 0}]);

    game.game_state.white_state.valid_moves = white_moves;

    let move_result = game.validate_move(
      &PieceMove {start: Position{row: 5, column: 0}, end: Position{row: 6, column: 0}, promotion: Some("Q".to_string())}
    );

    assert!(!move_result);
  }

  /**
   * Tests the validate_move_selection function with a promotion provided on a non-pawn move. Should return false.
   */
  #[test]
  fn invalid_piece_promotion_move_selection() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 6},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let mut game = Game::new(game_config);

    let mut white_moves = HashMap::new();
    white_moves.insert(Position{row: 6, column: 0}, vec![Position{row: 7, column: 0}]);

    game.game_state.white_state.valid_moves = white_moves;

    let move_result = game.validate_move(
      &PieceMove {start: Position{row: 6, column: 0}, end: Position{row: 7, column: 0}, promotion: Some("Q".to_string())}
    );

    assert!(!move_result);
  }
}

#[cfg(test)]
mod util_tests {
  use crate::{game::PositionalData, model::{CastlingState, Position, State}, move_data::MoveData, pieces::piece::Piece};

  use std::collections::HashMap;

  /**
   * Tests the get_state_change with an error scenario, where a king is missing from the board, to check the game state is updated correctly to State::Error.
   */
  #[test]
  fn error_state_change_scenario() {
    let mut board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, Some(Piece::King(true)), None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None]
    ];

    let state_change = super::get_state_change(
      &mut board, 
      true, 
      &CastlingState { long_castle: false, short_castle: false },
      &None
    );

    assert!(state_change.state == State::Error);
  }

  /**
   * Tests the get_state_change with an standard non-checking scenario to check the game state is updated correctly to State::Active.
   */
  #[test]
  fn active_state_change_scenario() {

    let mut board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::Rook(true)), None, Some(Piece::King(true)), None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, Some(Piece::King(false))], 
      vec![None, None, None, None, None, None, None, None]
    ];
    
    let state_change = super::get_state_change(
      &mut board, 
      true, 
      &CastlingState { long_castle: false, short_castle: false },
      &None
    );

    assert!(state_change.state == State::Active);
  }

  /**
   * Tests the get_state_change with an checkmate scenario to check the game state is updated correctly to State::BlackWin when white has no remaining valid moves and the white king is in check.
   */
  #[test]
  fn black_win_game_state_scenario() {
    let mut board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, Some(Piece::King(true)), None, None, None, None, None],
      vec![None, None, Some(Piece::Queen(false)), None, None, None, None, None], 
      vec![None, None, Some(Piece::King(false)), None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None]
    ];

    let state_change = super::get_state_change(
      &mut board, 
      true, 
      &CastlingState { long_castle: false, short_castle: false },
      &None
    );

    assert!(state_change.state == State::BlackWin);
  }

  /**
   * Tests the get_state_change with an checkmate scenario to check the game state is updated correctly to State::WhiteWin when black has no remaining valid moves and the black king is in check.
   */
  #[test]
  fn white_win_game_state_scenario() {
    let mut board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, Some(Piece::King(false)), None, None, None, None, None],
      vec![None, None, Some(Piece::Queen(true)), None, None, None, None, None], 
      vec![None, None, Some(Piece::King(true)), None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None]
    ];

    let state_change = super::get_state_change(
      &mut board, 
      false, 
      &CastlingState { long_castle: false, short_castle: false },
      &None
    );

    assert!(state_change.state == State::WhiteWin);
  }

  /**
   * Tests the get_state_change with an stalemate scenario to check the game state is updated correctly to State::Stalemate when the current player is not in check but has no valid moves.
   */
  #[test]
  fn stalemate_win_game_state_scenario() {
    let mut board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, Some(Piece::King(true)), None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, Some(Piece::Queen(true)), None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let state_change = super::get_state_change(
      &mut board, 
      false, 
      &CastlingState { long_castle: false, short_castle: false },
      &None
    );

    assert!(state_change.state == State::Draw);
  }

  /**
   * Tests the collect_positional_data function return data is correct for the provided board layout.
   */
  #[test]
  fn positional_data_collection_valid() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, Some(Piece::King(true)), None, None, None, None, None],
      vec![Some(Piece::King(false)), None, Some(Piece::Rook(true)), None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None]
    ];

    let positional_data = super::collect_positional_data(&board, false, &None).unwrap();

    assert!(positional_data.white_moves.len() == 2);
    assert!(positional_data.black_moves.len() == 1);

    for white_move_data in &positional_data.white_moves {
      if white_move_data.position.row == 0 && white_move_data.position.column == 2 { // King
        let expected_moves = vec![Position{row: 1, column: 3}, Position{row: 0, column: 3}];
        for position in &expected_moves {
          assert!(white_move_data.valid_moves.contains(position));
        }
        for position in &expected_moves {
          assert!(white_move_data.attacks.contains(position));
        }
        println!("{:?}", white_move_data.defends);
        let expected_defends = vec![Position{row: 1, column: 2}];
        for position in &expected_defends {
          assert!(white_move_data.defends.contains(position));
        }
        assert!(white_move_data.pins.is_empty());
        assert!(white_move_data.checking_path.is_none());
      } else if white_move_data.position.row == 1 && white_move_data.position.column == 2 { // Rook
        let expected_moves = vec![
          Position{row: 1, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 3}, Position{row: 1, column: 4},
          Position{row: 1, column: 5}, Position{row: 1, column: 6}, Position{row: 1, column: 7}, Position{row: 2, column: 2},
          Position{row: 3, column: 2}, Position{row: 4, column: 2}, Position{row: 5, column: 2}, Position{row: 6, column: 2}, Position{row: 7, column: 2},
        ];
        for position in &expected_moves {
          assert!(white_move_data.valid_moves.contains(position));
        }
        for position in &expected_moves {
          assert!(white_move_data.attacks.contains(position));
        }
        let expected_defends = vec![Position{row: 0, column: 2}];
        for position in &expected_defends {
          assert!(white_move_data.defends.contains(position));
        }
        assert!(white_move_data.pins.is_empty());
        assert!(white_move_data.checking_path.is_some());
        let path = white_move_data.checking_path.as_ref().unwrap();
        assert!(path.len() == 1);
        assert!(path[0] == Position{row: 1, column: 1});
      } else {
        assert!(false) // Element doesn't match expected
      }
    }

    let black_move_data = &positional_data.black_moves[0];
    assert!(black_move_data.position == Position{row: 1, column: 0});
    let expected_moves = vec![Position{row: 0, column: 0}, Position{row: 2, column: 0}];
    for position in &expected_moves {
      assert!(black_move_data.valid_moves.contains(position));
    }
    for position in &expected_moves {
      assert!(black_move_data.attacks.contains(position));
    }
    assert!(black_move_data.defends.is_empty());
    assert!(black_move_data.pins.is_empty());
    assert!(black_move_data.checking_path.is_none());
  }

  /**
   * Tests the is_checked function captures a check scenario.
   */
  #[test]
  fn check_scenario_captured() {
    let positional_data = PositionalData {
      white_moves: vec![
        MoveData { // King
          position: Position{row: 0, column: 2},
          valid_moves: vec![Position{row: 1, column: 3}, Position{row: 0, column: 3}], 
          attacks: vec![Position{row: 1, column: 3}, Position{row: 0, column: 3}], 
          defends: vec![], pins: vec![], checking_path: None
        },
        MoveData { // Rook
          position: Position{row: 1, column: 2},
          valid_moves: vec![
            Position{row: 1, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 3}, Position{row: 1, column: 4},
            Position{row: 1, column: 5}, Position{row: 1, column: 6}, Position{row: 1, column: 7}, Position{row: 2, column: 2},
            Position{row: 3, column: 2}, Position{row: 4, column: 2}, Position{row: 5, column: 2}, Position{row: 6, column: 2}, Position{row: 7, column: 2},
          ], 
          attacks: vec![
            Position{row: 1, column: 1}, Position{row: 1, column: 0}, Position{row: 1, column: 3}, Position{row: 1, column: 4},
            Position{row: 1, column: 5}, Position{row: 1, column: 6}, Position{row: 1, column: 7}, Position{row: 2, column: 2},
            Position{row: 3, column: 2}, Position{row: 4, column: 2}, Position{row: 5, column: 2}, Position{row: 6, column: 2}, Position{row: 7, column: 2},
          ], 
          defends: vec![Position{row: 0, column: 2}], pins: vec![], 
          checking_path: Some(vec![Position{row: 1, column: 1}])
        }
      ],
      black_moves: vec![
        MoveData {
          position: Position{row: 1, column: 0},
          valid_moves: vec![Position{row: 0, column: 0}, Position{row: 2, column: 0}], 
          attacks: vec![Position{row: 0, column: 0}, Position{row: 2, column: 0}], 
          defends: vec![], pins: vec![], checking_path: None
        }
      ],
      white_king_index: 0,
      black_king_index: 0
    };

    assert!(super::is_checked(false, &positional_data));
  }

  /**
   * Tests the update_king_valid_moves function for a correct modification to the valid moves list for the king.
   */
  #[test]
  fn correctly_adjust_king_moves() {
    let mut king_data = MoveData {
      position: Position{row: 5, column: 0},
      valid_moves: vec![
        Position{row: 6, column: 0}, Position{row: 6, column: 1},
        Position{row: 5, column: 1}, Position{row: 4, column: 1}, Position{row: 4, column: 0}
      ], 
      attacks: vec![
        Position{row: 6, column: 0}, Position{row: 6, column: 1},
        Position{row: 5, column: 1}, Position{row: 4, column: 1}, Position{row: 4, column: 0}
      ], 
      defends: vec![], 
      pins: vec![],
      checking_path: None
    };
    
    let mut opposing_pieces: Vec<MoveData> = vec![];

    opposing_pieces.push(MoveData { // Rook
      position: Position{row: 0, column: 1},
      valid_moves: vec![
        Position{row: 0, column: 0}, Position{row: 1, column: 1}, Position{row: 2, column: 1}, Position{row: 3, column: 1},
        Position{row: 4, column: 1}, Position{row: 5, column: 1}, Position{row: 6, column: 1}, Position{row: 7, column: 1}
      ], 
      attacks: vec![
        Position{row: 0, column: 0}, Position{row: 1, column: 1}, Position{row: 2, column: 1}, Position{row: 3, column: 1},
        Position{row: 4, column: 1}, Position{row: 5, column: 1}, Position{row: 6, column: 1}, Position{row: 7, column: 1}
      ], 
      defends: vec![], 
      pins: vec![],
      checking_path: None
    });

    opposing_pieces.push(MoveData { // Bishop
      position: Position{row: 2, column: 4},
      valid_moves: vec![
        Position{row: 1, column: 3}, 
        Position{row: 1, column: 5}, Position{row: 0, column: 6}, 
        Position{row: 3, column: 3}, Position{row: 4, column: 2}, Position{row: 5, column: 1}, Position{row: 6, column: 0}, 
        Position{row: 3, column: 5}, Position{row: 4, column: 6}, Position{row: 5, column: 7}
      ], 
      attacks: vec![
        Position{row: 1, column: 3}, 
        Position{row: 1, column: 5}, Position{row: 0, column: 6}, 
        Position{row: 3, column: 3}, Position{row: 4, column: 2}, Position{row: 5, column: 1}, Position{row: 6, column: 0}, 
        Position{row: 3, column: 5}, Position{row: 4, column: 6}, Position{row: 5, column: 7}
      ], 
      defends: vec![], 
      pins: vec![],
      checking_path: None
    });

    super::update_king_valid_moves(&mut king_data, &opposing_pieces);

    assert!(king_data.valid_moves.len() == 1);
    assert!(king_data.valid_moves[0] == Position{row: 4, column: 0});
  }

  /**
   * Tests the update_king_valid_moves function accounts for an attack which is invalid due to the piece being defended.
   */
  #[test]
  fn adjust_king_moves_with_defend() {
    let mut king_data = MoveData {
      position: Position{row: 7, column: 0},
      valid_moves: vec![
        Position{row: 7, column: 1}, Position{row: 6, column: 1}, Position{row: 6, column: 0}
      ], 
      attacks: vec![
        Position{row: 7, column: 1}, Position{row: 6, column: 1}, Position{row: 6, column: 0}
      ], 
      defends: vec![], 
      pins: vec![],
      checking_path: None
    };
    
    let mut opposing_pieces: Vec<MoveData> = vec![];

    opposing_pieces.push(MoveData { // Rook
      position: Position{row: 0, column: 1},
      valid_moves: vec![
        Position{row: 0, column: 0}, Position{row: 1, column: 1}, Position{row: 2, column: 1}, Position{row: 3, column: 1},
        Position{row: 4, column: 1}, Position{row: 5, column: 1}, Position{row: 6, column: 1}, Position{row: 7, column: 1}
      ], 
      attacks: vec![
        Position{row: 0, column: 0}, Position{row: 1, column: 1}, Position{row: 2, column: 1}, Position{row: 3, column: 1},
        Position{row: 4, column: 1}, Position{row: 5, column: 1}, Position{row: 6, column: 1}, Position{row: 7, column: 1}
      ], 
      defends: vec![Position{row: 7, column: 1}], 
      pins: vec![],
      checking_path: None
    });

    opposing_pieces.push(MoveData { // Knight
      position: Position{row: 7, column: 1},
      valid_moves: vec![
        Position{row: 5, column: 0}, Position{row: 5, column: 2}, Position{row: 6, column: 3}
      ], 
      attacks: vec![
        Position{row: 5, column: 0}, Position{row: 5, column: 2}, Position{row: 6, column: 3}
      ], 
      defends: vec![], 
      pins: vec![],
      checking_path: None
    });

    super::update_king_valid_moves(&mut king_data, &opposing_pieces);

    assert!(king_data.valid_moves.len() == 1);
    assert!(king_data.valid_moves[0] == Position{row: 6, column: 0});
  }

  /**
   * Tests the king_is_attacked function to make sure it returns true when the king is currently under attack.
   */
  #[test]
  fn king_is_attacked_check() {
    let mut opposing_pieces: Vec<MoveData> = vec![];

    opposing_pieces.push(MoveData {
      position: Position{row: 5, column: 2},
      valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![],
      checking_path: Some(vec![])
    });

    assert!(super::king_is_attacked(&opposing_pieces))
  }

  /**
   * Tests the king_is_attacked function to make sure it returns false when the king is not under attack.
   */
  #[test]
  fn king_not_attacked_check() {
    let mut opposing_pieces: Vec<MoveData> = vec![];

    opposing_pieces.push(MoveData {
      position: Position{row: 4, column: 3},
      valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![],
      checking_path: None
    });

    opposing_pieces.push(MoveData {
      position: Position{row: 0, column: 0},
      valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![],
      checking_path: None
    });

    assert!(!super::king_is_attacked(&opposing_pieces))
  }

  /**
   * Tests the get_pinned_position_map function to ensure a correct pin mapping is created.
   */
  #[test]
  fn pinned_position_mapping() {
    let mut attacking_pieces: Vec<MoveData> = vec![];

    attacking_pieces.push(MoveData {
      position: Position{row: 7, column: 6},
      valid_moves: vec![], attacks: vec![], defends: vec![],
      pins: vec![Position {row: 7, column: 2}],
      checking_path: None
    });

    attacking_pieces.push(MoveData {
      position: Position{row: 2, column: 0},
      valid_moves: vec![], attacks: vec![], defends: vec![],
      pins: vec![Position {row: 5, column: 0}],
      checking_path: None
    });

    let pinned_positions: HashMap<Position, Position> = super::get_pinned_position_map(&attacking_pieces);
    
    assert!(pinned_positions.len() == 2);
    assert!(pinned_positions.contains_key(&Position {row: 5, column: 0}));
    
    let mut value = pinned_positions.get(&Position {row: 5, column: 0});
    assert!(value.is_some());
    assert!(value.unwrap().row == 2 as usize);
    assert!(value.unwrap().column == 0 as usize);
    
    value = pinned_positions.get(&Position {row: 7, column: 2});
    assert!(value.is_some());
    assert!(value.unwrap().row == 7 as usize);
    assert!(value.unwrap().column == 6 as usize);
  }

  /**
   * Tests the adjust_pinned_valid_moves to make sure that it correctly identifies which moves are invalid and which are still valid based on a pin.
   */
  #[test]
  fn pinned_piece_valid_moves_altered() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, None, None, None, None, None, Some(Piece::King(true))],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Queen(true)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Rook(false)), None, None, None, None, None, None], 
      vec![None, Some(Piece::King(false)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None]
    ];

    let attacking_piece_position = Position {row: 2, column: 1};

    let rook_move_data = MoveData{
      position: Position {row: 5, column: 1},
      valid_moves: vec![
        Position {row: 5, column: 0}, 
        Position {row: 5, column: 2}, Position {row: 5, column: 3}, Position {row: 5, column: 4},
        Position {row: 5, column: 5}, Position {row: 5, column: 6}, Position {row: 5, column: 7}, 
        Position {row: 4, column: 1}, Position {row: 3, column: 1}, Position {row: 2, column: 1}
      ],
      attacks: vec![],
      defends: vec![],
      pins: vec![],
      checking_path: None
    };
  
    let adjusted_positions: Vec<Position> = super::adjust_pinned_valid_moves(&rook_move_data, &attacking_piece_position, &board);

    let expected_moves: Vec<Position> = vec![Position {row: 4, column: 1}, Position {row: 3, column: 1}, Position {row: 2, column: 1}];

    assert!(adjusted_positions.len() == expected_moves.len());

    let mut matching: bool = true;
    for position in &adjusted_positions {
      if !expected_moves.contains(position) {
        matching = false;
      }
    }

    assert!(matching);
  }

  /**
   * Tests the adjust_pinned_valid_moves to make sure that it correctly identifies a pinned piece that has no valid moves.
   */
  #[test]
  fn pinned_piece_no_valid_moves() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![None, None, None, None, None, None, None, Some(Piece::King(true))],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Queen(true)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Bishop(false)), None, None, None, None, None, None], 
      vec![None, Some(Piece::King(false)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None]
    ];

    let attacking_piece_position = Position {row: 2, column: 1};

    let bishop_move_data = MoveData{
      position: Position {row: 5, column: 1},
      valid_moves: vec![
        Position {row: 6, column: 0}, 
        Position {row: 6, column: 2}, Position {row: 7, column: 3}, 
        Position {row: 4, column: 0},
        Position {row: 4, column: 2}, Position {row: 3, column: 3}, Position {row: 2, column: 4}, 
        Position {row: 1, column: 5}, Position {row: 0, column: 6}
      ],
      attacks: vec![],
      defends: vec![],
      pins: vec![],
      checking_path: None
    };
  
    let adjusted_positions: Vec<Position> = super::adjust_pinned_valid_moves(&bishop_move_data, &attacking_piece_position, &board);

    assert!(adjusted_positions.is_empty());
  }

  /**
   * Tests the has_sufficient_material function for a board where a player does have sufficient material to checkmate the opponent (has a queen).
   */
  #[test]
  fn valid_sufficient_material_queen() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Queen(true)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 1},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(super::has_sufficient_material(&moves, &board));
  }

  /**
   * Tests the has_sufficient_material function for a board where a player does have sufficient material to checkmate the opponent (has a rook).
   */
  #[test]
  fn valid_sufficient_material_rook() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Rook(true)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 1},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(super::has_sufficient_material(&moves, &board));
  }

  /**
   * Tests the has_sufficient_material function for a board where a player does have sufficient material to checkmate the opponent (has a pawn).
   */
  #[test]
  fn valid_sufficient_material_pawn() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, Some(Piece::Pawn(true)), None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 1},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(super::has_sufficient_material(&moves, &board));
  }

  /**
   * Tests the has_sufficient_material function for a board where a player does have sufficient material to checkmate the opponent (2 bishops of different board colours, light and dark).
   */
  #[test]
  fn valid_sufficient_material_two_different_coloured_bishops() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![Some(Piece::Bishop(true)), None, None, None, None, None, None, None], 
      vec![Some(Piece::Bishop(true)), None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 1, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(super::has_sufficient_material(&moves, &board));
  }

  /**
   * Tests the has_sufficient_material function for a board where a player does have sufficient material to checkmate the opponent (3 knights are sufficient).
   */
  #[test]
  fn valid_sufficient_material_three_knights() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, Some(Piece::Knight(true)), None, None, None, None], 
      vec![Some(Piece::Knight(true)), None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, Some(Piece::Knight(true))], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 1, column: 3},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 3, column: 7},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(super::has_sufficient_material(&moves, &board));
  }

  /**
   * Tests the has_sufficient_material function for a board where a player doesn't have sufficient material to checkmate the opponent (only a king).
   */
  #[test]
  fn invalid_sufficient_material_king_only() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(!super::has_sufficient_material(&moves, &board));
  }

  /**
   * Tests the has_sufficient_material function for a board where a player doesn't have sufficient material to checkmate the opponent (2 knights are insufficient).
   */
  #[test]
  fn invalid_sufficient_material_two_knights() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, Some(Piece::Knight(true)), None, None, None, None], 
      vec![Some(Piece::Knight(true)), None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 1, column: 3},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(!super::has_sufficient_material(&moves, &board));
  }

    /**
   * Tests the has_sufficient_material function for a board where a player doesn't have sufficient material to checkmate the opponent (2 bishops of the same board colour).
   */
  #[test]
  fn invalid_sufficient_material_two_bishops_same_colour() {
    let board: Vec<Vec<Option<Piece>>> = vec![
      vec![Some(Piece::King(true)), None, None, None, None, None, None, None],
      vec![None, None, None, Some(Piece::Bishop(true)), None, None, None, None], 
      vec![Some(Piece::Bishop(true)), None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![None, None, None, None, None, None, None, None], 
      vec![Some(Piece::King(false)), None, None, None, None, None, None, None]
    ];

    let moves: Vec<MoveData> = vec![
      MoveData{
        position: Position {row: 0, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 1, column: 3},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      },
      MoveData{
        position: Position {row: 2, column: 0},
        valid_moves: vec![], attacks: vec![], defends: vec![], pins: vec![], checking_path: None
      }
    ];

    assert!(!super::has_sufficient_material(&moves, &board));
  }
}