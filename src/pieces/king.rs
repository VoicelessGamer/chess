use std::ops::Range;

use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::piece_util::piece_util::examine_position, piece_move::PieceMove
};

/**
 * Retrieves the relevant move data for a King piece at a given position on the board.
 * This move data contains all the currently valid moves, positions under attack and friendly pieces defended by this piece.
 */
pub fn get_king_move_data(origin: &Position, board: &Vec<Vec<Option<Piece>>>) -> MoveData {
  let mut valid_moves: Vec<Position> = vec![]; // Valid positions this piece can move to including captures
  let mut attacks: Vec<Position> = vec![];              // Valid positions this piece has under attack
  let mut defends: Vec<Position> = vec![]; // Friendly pieces defended by this piece

  let is_white = board[origin.row][origin.column].as_ref().unwrap().is_white();

  let row = origin.row as i8;
  let column = origin.column as i8;

  // Examine each possible position for a king
  examine_position(row + 1, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row - 1, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row + 1, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row - 1, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row - 1, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);
  examine_position(row + 1, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut false);

  return MoveData {
    position: origin.clone(),
    valid_moves,
    attacks,
    defends,
    pins: vec![], // Kings cannot pin
    checking_path: None // Kings cannot check
  }
}

/**
 * Checks whether the given board state means that the king at the origin position can castle long
 */
pub fn is_king_long_castle_valid(origin: &Position, board: &Vec<Vec<Option<Piece>>>, attacked_positions: &Vec<Position>) -> bool {
  let mut is_valid = is_castle_valid(origin.row, 2..origin.column, board, attacked_positions);

  // For a long castle, also need to check there is no blocking piece on the 2nd File/column.
  if is_valid {
    is_valid = !attacked_positions.contains(origin) && !board[origin.row][1].is_some();
  }

  return is_valid;
}

/**
 * Checks whether the given board state means that the king at the origin position can castle short
 */
pub fn is_king_short_castle_valid(origin: &Position, board: &Vec<Vec<Option<Piece>>>, attacked_positions: &Vec<Position>) -> bool {
  let mut is_valid =  is_castle_valid(origin.row, origin.column + 1..7, board, attacked_positions);

  if is_valid {
    is_valid = !attacked_positions.contains(origin);
  }

  return is_valid;
}

/**
 * Checks all positions on a given row and each column in the columns parameters for empty, non-attacked positions
 * Returns true if all checked positions must be free and not under attack
 */
fn is_castle_valid(row: usize, columns: Range<usize>, board: &Vec<Vec<Option<Piece>>>, attacked_positions: &Vec<Position>) -> bool {
  for i in columns {
    if board[row][i].is_some() || attacked_positions.contains(&Position {row, column: i}) {
      return false;
    }
  }

  return true;
}

/**
 * Checks the player's move to see if it was a castling move (king moving 2 spaces).
 * If it was a castling move then the move for the Rook is returned.
 * This assumes that the move has already been validated.
 */
pub fn get_castle_move(piece_move: &PieceMove, board: &Vec<Vec<Option<Piece>>>) -> Option<PieceMove> {
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

#[cfg(test)]
mod king_tests {
  use crate::{config::{PieceConfig, self}, board::Board, position::Position, pieces::king::*};

  /**
   * Testing the attacks have all been calculated correctly through the get_king_move_data function when all positions are not under attack
   */
  #[test]
  fn test_standard_positions() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 3, row: 3}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 3, column: 3};
    let move_data = get_king_move_data(&pos, &current_board);

    assert_eq!(move_data.attacks.len(), 8);
    assert!(move_data.attacks.contains(&Position {row: 4, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 2}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 2}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 2}));
  }

  /**
   * Testing the attacks and defends have all been calculated correctly through the get_king_move_data function
   */
  #[test]
  fn test_attack_defend() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 1},
        PieceConfig {piece: String::from("king"), white: true, column: 0, row: 0},
        PieceConfig {piece: String::from("pawn"), white: false, column: 1, row: 1},
        PieceConfig {piece: String::from("king"), white: false, column: 0, row: 3}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 0, column: 0};
    let move_data = get_king_move_data(&pos, &current_board);

    assert_eq!(move_data.position, Position {row: 0, column: 0});

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 1, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 0, column: 1}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 1, column: 0}));

    assert!(move_data.pins.is_empty());

    assert!(move_data.checking_path.is_none());
  }

  /**
   * Testing the is_king_long_castle_valid returns true when all positions are clear between the king and the rook and are not attacked
   */
  #[test]
  fn valid_long_castle_black() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_long_castle_valid(&Position{row:7, column:4}, current_board, &vec![]), true);
  }

  /**
   * Testing the is_king_long_castle_valid returns true when the c8 and d8 positions are clear between the king and the rook,
   * but the b8 position is under attack
   */
  #[test]
  fn valid_long_castle_black_b8_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("rook"), white: true, column: 1, row: 0},
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:1}];

    assert_eq!(is_king_long_castle_valid(&Position{row:7, column:4}, current_board, &attacks), true);
  }

  /**
   * Testing the is_king_long_castle_valid returns true when all positions are clear between the king and the rook and are not attacked
   */
  #[test]
  fn valid_long_castle_white() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_long_castle_valid(&Position{row:0, column:4}, current_board, &vec![]), true);
  }

  /**
   * Testing the is_king_long_castle_valid returns true when the c1 and d1 positions are clear between the king and the rook,
   * but the b1 position is under attack
   */
  #[test]
  fn valid_long_castle_white_b1_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("rook"), white: false, column: 1, row: 7},
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:1}];

    assert_eq!(is_king_long_castle_valid(&Position{row:0, column:4}, current_board, &attacks), true);
  }

  /**
   * Testing the is_king_short_castle_valid returns true when all positions are clear between the king and the rook and are not attacked
   */
  #[test]
  fn valid_short_castle_white() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_short_castle_valid(&Position{row:0, column:4}, current_board, &vec![]), true);
  }

  /**
   * Testing the is_king_short_castle_valid returns true when all positions are clear between the king and the rook and are not attacked
   */
  #[test]
  fn valid_short_castle_black() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_short_castle_valid(&Position{row:7, column:4}, current_board, &vec![]), true);
  }

    /**
   * Testing the is_king_long_castle_valid returns true when the rook is under attack
   */
  #[test]
  fn valid_long_castle_white_rook_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:0}];

    assert_eq!(is_king_long_castle_valid(&Position{row:0, column:4}, current_board, &attacks), true);
  }

  /**
   * Testing the is_king_long_castle_valid returns true when the rook is under attack
   */
  #[test]
  fn valid_long_castle_black_rook_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:0}];

    assert_eq!(is_king_long_castle_valid(&Position{row:7, column:4}, current_board, &attacks), true);
  }

  /**
   * Testing the is_king_short_castle_valid returns true when the rook is under attack
   */
  #[test]
  fn valid_short_castle_white_rook_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:7}];

    assert_eq!(is_king_short_castle_valid(&Position{row:0, column:4}, current_board, &attacks), true);
  }

  /**
   * Testing the is_king_short_castle_valid returns true when the rook is under attack
   */
  #[test]
  fn valid_short_castle_black_rook_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:7}];

    assert_eq!(is_king_short_castle_valid(&Position{row:7, column:4}, current_board, &attacks), true);
  }

  /**
   * Testing the is_king_long_castle_valid returns false when the king and the rook are blocked
   */
  #[test]
  fn invalid_long_castle_white_blocked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("knight"), white: true, column: 1, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_long_castle_valid(&Position{row:0, column:4}, current_board, &vec![]), false);
  }

  /**
   * Testing the is_king_long_castle_valid returns false when the king and the rook are blocked
   */
  #[test]
  fn invalid_long_castle_black_blocked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("knight"), white: false, column: 1, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_long_castle_valid(&Position{row:7, column:4}, current_board, &vec![]), false);
  }

  /**
   * Testing the is_king_short_castle_valid returns false when the king and the rook are blocked
   */
  #[test]
  fn invalid_short_castle_white_blocked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("knight"), white: true, column: 6, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_short_castle_valid(&Position{row:0, column:4}, current_board, &vec![]), false);
  }

  /**
   * Testing the is_king_short_castle_valid returns false when the king and the rook are blocked
   */
  #[test]
  fn invalid_short_castle_black_blocked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("knight"), white: false, column: 6, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    assert_eq!(is_king_short_castle_valid(&Position{row:7, column:4}, current_board, &vec![]), false);
  }

  /**
   * Testing the is_king_long_castle_valid returns false when a king's move position between the king and the rook is attacked
   */
  #[test]
  fn invalid_long_castle_white_attacked_position() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 3, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:3}];

    assert_eq!(is_king_long_castle_valid(&Position{row:0, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_long_castle_valid returns false when a king's move position between the king and the rook is attacked
   */
  #[test]
  fn invalid_long_castle_black_attacked_position() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7},
        PieceConfig {piece: String::from("rook"), white: true, column: 3, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:3}];

    assert_eq!(is_king_long_castle_valid(&Position{row:7, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_short_castle_valid returns false when a king's move position between the king and the rook is attacked
   */
  #[test]
  fn invalid_short_castle_white_attacked_position() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},
        PieceConfig {piece: String::from("rook"), white: false, column: 5, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:5}];

    assert_eq!(is_king_short_castle_valid(&Position{row:0, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_short_castle_valid returns false when a king's move position between the king and the rook is attacked
   */
  #[test]
  fn invalid_short_castle_black_attacked_position() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 5, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:5}];

    assert_eq!(is_king_short_castle_valid(&Position{row:7, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_long_castle_valid returns false when a king is under attack
   */
  #[test]
  fn invalid_long_castle_white_king_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 4, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:4}];

    assert_eq!(is_king_long_castle_valid(&Position{row:0, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_long_castle_valid returns false when a king is under attack
   */
  #[test]
  fn invalid_long_castle_black_king_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7},
        PieceConfig {piece: String::from("rook"), white: true, column: 4, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:4}];

    assert_eq!(is_king_long_castle_valid(&Position{row:7, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_short_castle_valid returns false when a king is under attack
   */
  #[test]
  fn invalid_short_castle_white_king_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},
        PieceConfig {piece: String::from("rook"), white: false, column: 4, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:0, column:4}];

    assert_eq!(is_king_short_castle_valid(&Position{row:0, column:4}, current_board, &attacks), false);
  }

  /**
   * Testing the is_king_short_castle_valid returns false when a king is under attack
   */
  #[test]
  fn invalid_short_castle_black_king_attacked() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 4, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    // Excluding unnecessary positions from attacks vector for simplicity
    let attacks = vec![Position{row:7, column:4}];

    assert_eq!(is_king_short_castle_valid(&Position{row:7, column:4}, current_board, &attacks), false);
  }

  /**
   * Tests the get_castle_move function return the correct rook piece move for a long castle
   */
  #[test]
  fn some_rook_move_long_castle_black() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:7, column:4}, end: Position{row:7, column:2}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_some());

    let r_move = rook_move.as_ref().unwrap();
    assert_eq!(r_move.start, Position{row:7, column:0});
    assert_eq!(r_move.end, Position{row:7, column:3});
  }

  /**
   * Tests the get_castle_move function return the correct rook piece move for a long castle
   */
  #[test]
  fn some_rook_move_long_castle_white() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:0, column:4}, end: Position{row:0, column:2}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_some());

    let r_move = rook_move.as_ref().unwrap();
    assert_eq!(r_move.start, Position{row:0, column:0});
    assert_eq!(r_move.end, Position{row:0, column:3});
  }

  /**
   * Tests the get_castle_move function return the correct rook piece move for a short castle
   */
  #[test]
  fn some_rook_move_short_castle_black() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:7, column:4}, end: Position{row:7, column:6}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_some());

    let r_move = rook_move.as_ref().unwrap();
    assert_eq!(r_move.start, Position{row:7, column:7});
    assert_eq!(r_move.end, Position{row:7, column:5});
  }

  /**
   * Tests the get_castle_move function return the correct rook piece move for a short castle
   */
  #[test]
  fn some_rook_move_short_castle_white() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:0, column:4}, end: Position{row:0, column:6}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_some());

    let r_move = rook_move.as_ref().unwrap();
    assert_eq!(r_move.start, Position{row:0, column:7});
    assert_eq!(r_move.end, Position{row:0, column:5});
  }

  /**
   * Tests the get_castle_move function returns none for a long castle where the king didn't move 2 spaces
   */
  #[test]
  fn none_rook_move_long_castle_black_no_castle() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:7, column:4}, end: Position{row:7, column:3}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a long castle where the king didn't move 2 spaces
   */
  #[test]
  fn none_rook_move_long_castle_white_no_castle() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:0, column:4}, end: Position{row:0, column:3}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a short castle where the king didn't move 2 spaces
   */
  #[test]
  fn none_rook_move_short_castle_black_no_castle() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:7, column:4}, end: Position{row:7, column:5}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a short castle where the king didn't move 2 spaces
   */
  #[test]
  fn none_rook_move_short_castle_white_no_castle() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:0, column:4}, end: Position{row:0, column:5}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a long castle where the move was not made by the king
   */
  #[test]
  fn none_rook_move_long_castle_black_not_king() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7},
        PieceConfig {piece: String::from("pawn"), white: false, column: 0, row: 6}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:6, column:0}, end: Position{row:5, column:0}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a long castle where the move was not made by the king
   */
  #[test]
  fn none_rook_move_long_castle_white_not_king() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
        PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 1}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:1, column:0}, end: Position{row:2, column:0}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a short castle where the move was not made by the king
   */
  #[test]
  fn none_rook_move_short_castle_black_not_king() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7},
        PieceConfig {piece: String::from("pawn"), white: false, column: 0, row: 6}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:6, column:0}, end: Position{row:5, column:0}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

  /**
   * Tests the get_castle_move function returns none for a short castle where the move was not made by the king
   */
  #[test]
  fn none_rook_move_short_castle_white_not_king() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},
        PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 1}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = &board.get_current_board();

    let piece_move = PieceMove { start: Position{row:1, column:0}, end: Position{row:2, column:0}, promotion: None };

    let rook_move = get_castle_move(&piece_move, current_board);

    assert!(rook_move.is_none());
  }

}
