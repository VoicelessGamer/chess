use std::collections::HashMap;

use crate::{
  piece_move::PieceMove, 
  pieces::piece::Piece, 
  game::{GameState, State}, 
  position::Position
};


#[allow(dead_code)] // TODO:Remove
#[derive(Clone)]
pub struct LoggedMove {
  piece_move: PieceMove,
  pub pgn_notation: String
}

/**
 * The 'moves' field is a vector of 2-element vectors. 
 * The first element is white's move, the second, black's
 */
#[allow(dead_code)] // TODO:Remove
pub struct MoveLogger {
  initial_board: Vec<Vec<Option<Piece>>>, // The state of the board at the beginning of the game
  pub moves: Vec<Vec<LoggedMove>>
}

impl MoveLogger {

  pub fn new(initial_board: Vec<Vec<Option<Piece>>>) -> Self {
    Self {
      initial_board,
      moves: vec![vec![]]
    }
  }

  /**
   * Adds a LoggedMove to the moves vector.
   * This function should be called after a move has been made and the game state is updated.
   */
  pub fn add_move(&mut self, piece_move: PieceMove, board: &Vec<Vec<Option<Piece>>>, game_state: &GameState) {
    let last = self.moves.len() - 1;
    if self.moves.len() == 0 || self.moves[last].len() == 2 {
      self.moves.push(vec![LoggedMove {pgn_notation: calculate_pgn(&piece_move, &board, &game_state), piece_move}]);
    } else {
      self.moves[last].push(LoggedMove {pgn_notation: calculate_pgn(&piece_move, &board, &game_state), piece_move});
    }
  }
}

/**
 * Calculates the standard pgn notation for a given move.
 */
fn calculate_pgn(piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>, game_state: &GameState) -> String {
  let piece = board[piece_move.end.row][piece_move.end.column].as_ref().unwrap();

  // Check for castling move which follow a separate marking structure
  let mut pgn = get_castling_notation(&piece, piece_move);

  // 'pgn' will be empty if it was not a castling move
  // Now update 'pgn' with standard move notation
  if pgn.is_empty() {
    // Add the standard piece abbreviation
    if piece_move.promotion.is_none() {
      pgn.push_str(get_piece_abbreviation(&piece));
    }

    // Check for piece ambiguity
    let ambiguity = check_ambiguity(&piece, piece_move, board, &game_state.valid_moves);

    if ambiguity.0 {
      // Add File for the ambiguity notation
      pgn.push(get_file_mapping(piece_move.start.column));
    } else if ambiguity.1 {
      // Add Rank for the ambiguity notation
      pgn.push(get_rank_mapping(piece_move.start.row));
    }

    // Add the target destination
    pgn.push(get_file_mapping(piece_move.end.column));
    pgn.push(get_rank_mapping(piece_move.end.row));

    // Add promotion notation, if necessary
    if piece_move.promotion.is_some() {
      pgn.push('=');
      pgn.push_str(piece_move.promotion.as_ref().unwrap().as_str());
    }
  }

  // Add check / checkmate marks, if required
  match game_state.state {
    State::BlackWin | State::WhiteWin => pgn.push('#'),
    _ => {
      if game_state.in_check {
        pgn.push('+')
      }
    }
  }

  return pgn;
}

/**
 * Checks whether a move was a castling move and returns the standard pgn castling notation.
 */
fn get_castling_notation(piece: &Piece, piece_move: &PieceMove) -> String {
  match piece {
    Piece::King(_) => {
      let column = piece_move.start.column;
      let target_column = piece_move.end.column;
      if target_column > column && target_column - column == 2 {
        // King-side castling move
        String::from("O-O")
      } else if column > target_column && column - target_column == 2 {
        // Queen-side castling move
        String::from("O-O-O")
      } else {
        String::from("")
      }
    },
    _ => String::from("")
  }
}

/**
 * Checks for any ambiguity in a move and returns a tuple to determine what should be added to the pgn notation to disambiguate
 * The return type is a tuple where position 0 means to add the File identifier and position 1 to add the Rank identifier
 */
fn check_ambiguity(piece: &Piece, piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>, valid_moves: &HashMap<Position, Vec<Position>>) -> (bool, bool) {
  match piece {
    Piece::Bishop(_) | 
    Piece::Knight(_) | 
    Piece::Pawn(_) | 
    Piece::Queen(_) | 
    Piece::Rook(_) => {
      for i in 0..board.len() {
        for j in 0..board[i].len() {
          // Ignore empty position
          if board[i][j].is_none() {
            continue;
          }

          // Check if the piece is of the same type and colour
          let checking_piece = board[i][j].as_ref().unwrap();
          if checking_piece != piece {
            continue;
          }

          let checking_pos = Position {row: i, column: j};
          if valid_moves.contains_key(&checking_pos) && valid_moves.get(&checking_pos).unwrap().contains(&piece_move.end) {
            // Some form of ambiguity exists
            if piece_move.start.column == j {
              // Both pieces were on the same File, need to display Rank in notation
              return (false, true);
            } else {
              // Regardless of if the pieces were on the same Rank, the File is the preferrence for ambiguity
              return (true, false);
            }
          }
        }
      }
      (false, false)
    },
    Piece::King(_) => (false, false) // No possibility of ambiguity,
  }
}

/**
 * Returns the standard pgn piece notation based on the supplied Piece
 */
fn get_piece_abbreviation(piece: &Piece) -> &str {
  match piece  {
    Piece::Bishop(_) => "B",
    Piece::King(_) => "K",
    Piece::Knight(_) => "N",
    Piece::Pawn(_) => "",
    Piece::Queen(_) => "Q",
    Piece::Rook(_) => "R"
  }
}

/**
 * Returns the alphabetic File notation for the supplied column
 */
fn get_file_mapping(column: usize) -> char {
  match column  {
    0 => 'a',
    1 => 'b',
    2 => 'c',
    3 => 'd',
    4 => 'e',
    5 => 'f',
    6 => 'g',
    7 => 'h',
    _ => ' ' // Unreachable option
  }
}

/**
 * Returns the row index, incremented, as a char
 */
fn get_rank_mapping(row: usize) -> char {
  match row  {
    0 => '1',
    1 => '2',
    2 => '3',
    3 => '4',
    4 => '5',
    5 => '6',
    6 => '7',
    7 => '8',
    _ => ' ' // Unreachable option
  }
}