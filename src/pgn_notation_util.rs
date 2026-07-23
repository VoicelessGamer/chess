use std::collections::HashMap;

use crate::{
  model::PieceMove, 
  pieces::piece::Piece, 
  model::{GameState, State}, 
  model::Position
};

/**
 * Calculates the standard pgn notation for a given move.
 */
pub fn calculate_pgn(piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>, game_state: &GameState) -> Option<String> {
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
    let valid_moves = match game_state.white_turn  {
      true => &game_state.white_state.valid_moves,
      false => &game_state.black_state.valid_moves
    };

    let ambiguity = check_ambiguity(&piece, piece_move, board, valid_moves);

    if ambiguity.0 {
      // Add File for the ambiguity notation
      pgn.push(get_file_mapping(piece_move.start.column)?);
    } else if ambiguity.1 {
      // Add Rank for the ambiguity notation
      pgn.push(get_rank_mapping(piece_move.start.row)?);
    }

    // Add the target destination
    pgn.push(get_file_mapping(piece_move.end.column)?);
    pgn.push(get_rank_mapping(piece_move.end.row)?);

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
      if (game_state.white_turn && game_state.white_state.in_check) || (!game_state.white_turn && game_state.white_state.in_check) {
        pgn.push('+')
      }
    }
  }

  return Some(pgn);
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
fn get_file_mapping(column: usize) -> Option<char> {
  match column  {
    0 => Some('a'),
    1 => Some('b'),
    2 => Some('c'),
    3 => Some('d'),
    4 => Some('e'),
    5 => Some('f'),
    6 => Some('g'),
    7 => Some('h'),
    _ => None // Unreachable option
  }
}

/**
 * Returns the row index, incremented, as a char
 */
fn get_rank_mapping(row: usize) -> Option<char> {
  match row  {
    0 => Some('1'),
    1 => Some('2'),
    2 => Some('3'),
    3 => Some('4'),
    4 => Some('5'),
    5 => Some('6'),
    6 => Some('7'),
    7 => Some('8'),
    _ => None // Unreachable option
  }
}

// TODO: Fill out the test suite for move logger functions.

#[cfg(test)]
mod util_tests {
  /**
   * Tests the get_file_mapping function returns the correct character.
   */
  #[test]
  fn valid_file_mapping() {
    let mut result = super::get_file_mapping(0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'a');

    result = super::get_file_mapping(1);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'b');

    result = super::get_file_mapping(2);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'c');

    result = super::get_file_mapping(3);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'd');

    result = super::get_file_mapping(4);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'e');

    result = super::get_file_mapping(5);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'f');

    result = super::get_file_mapping(6);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'g');

    result = super::get_file_mapping(7);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), 'h');
  }

  /**
   * Tests the get_file_mapping function returns none for file indices larger than 7.
   */
  #[test]
  fn invalid_file_mapping() {
    let result = super::get_file_mapping(8);
    assert!(result.is_none());
  }

  /**
   * Tests the get_rank_mapping function returns the correct rank index.
   */
  #[test]
  fn valid_rank_mapping() {
    let mut result = super::get_rank_mapping(0);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '1');

    result = super::get_rank_mapping(1);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '2');

    result = super::get_rank_mapping(2);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '3');

    result = super::get_rank_mapping(3);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '4');

    result = super::get_rank_mapping(4);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '5');

    result = super::get_rank_mapping(5);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '6');

    result = super::get_rank_mapping(6);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '7');

    result = super::get_rank_mapping(7);
    assert!(result.is_some());
    assert_eq!(result.unwrap(), '8');
  }

  /**
   * Tests the get_rank_mapping function returns none for rank indices larger than 7.
   */
  #[test]
  fn invalid_rank_mapping() {
    let result = super::get_rank_mapping(8);
    assert!(result.is_none());
  }
}