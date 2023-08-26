use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::piece_util::piece_util::examine_position
};

/**
 * Retrieves the relevant move data for a Knight piece at a given position on the board.
 * This move data contains all the currently valid moves, positions under attack, friendly pieces defended by this piece
 * and the path to the opposing king if it is in check by this piece.
 */
pub fn get_knight_move_data(origin: &Position, board: &Vec<Vec<Option<Piece>>>) -> MoveData {
  let mut valid_moves: Vec<Position> = vec![];          // Valid positions this piece can move to including captures
  let mut attacks: Vec<Position> = vec![];              // Valid positions this piece has under attack
  let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
  let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible
  let mut checking = false;

  let is_white = board[origin.row][origin.column].as_ref().unwrap().is_white();

  let row = origin.row as i8;
  let column = origin.column as i8;

  // Examine each possible position for a knight
  examine_position(row + 2, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row + 1, column + 2, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row - 1, column + 2, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row - 2, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row - 2, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row - 1, column - 2, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row + 1, column - 2, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  examine_position(row + 2, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
  
  if checking {
    checking_path = Some(vec![]);
  }

  return MoveData {
    position: origin.clone(),
    valid_moves,
    attacks,
    defends,
    pins: vec![], // Knights cannot pin
    checking_path
  }
}

#[cfg(test)]
mod knight_tests {
  use crate::{config::{PieceConfig, self}, board::Board, position::Position, pieces::knight::*};

  /**
   * Testing the attacks have all been calculated correctly through the get_knight_move_data function when all positions are not under attack
   */
  #[test]
  fn test_standard_positions() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("knight"), white: true, column: 3, row: 3}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 3, column: 3};
    let move_data = get_knight_move_data(&pos, &current_board);

    assert_eq!(move_data.attacks.len(), 8);
    assert!(move_data.attacks.contains(&Position {row: 5, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 2}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 5, column: 2}));
  }
 
  /**
   * Testing the attacks and defends have all been calculated correctly through the get_knight_move_data function
   */
  #[test]
  fn test_attack_defend() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 4, row: 0},
        PieceConfig {piece: String::from("knight"), white: true, column: 2, row: 1},
        PieceConfig {piece: String::from("pawn"), white: false, column: 3, row: 3},
        PieceConfig {piece: String::from("king"), white: false, column: 2, row: 3}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 1, column: 2};
    let move_data = get_knight_move_data(&pos, &current_board);

    assert_eq!(move_data.position, Position {row: 1, column: 2});

    assert_eq!(move_data.attacks.len(), 5);
    assert!(move_data.attacks.contains(&Position {row: 0, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 4}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 0, column: 4}));

    assert!(move_data.pins.is_empty());

    assert!(move_data.checking_path.is_none());
  }

  /**
   * Testing the checking path is calculated correctly through the get_knight_move_data function
   */
  #[test]
  fn test_check_path() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("knight"), white: true, column: 0, row: 1},
        PieceConfig {piece: String::from("king"), white: false, column: 2, row: 2}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 1, column: 0};
    let move_data = get_knight_move_data(&pos, &current_board);

    assert!(move_data.checking_path.is_some());

    // If checking the king there is no checking path for a knight as it cannot be blocked
    assert!(move_data.checking_path.unwrap().is_empty());
  }
}
