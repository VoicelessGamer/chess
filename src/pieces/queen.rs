use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::piece_util::piece_util::examine_line
};

/**
 * Retrieves the relevant move data for a Queen piece at a given position on the board.
 * This move data contains all the currently valid moves, positions under attack, friendly pieces defended by this piece,
 * opposing pieces pinned to the opposing king and the path to the opposing king if it is in check by this piece.
 */
pub fn get_queen_move_data(origin: &Position, board: &Vec<Vec<Option<Piece>>>) -> MoveData {
  let mut valid_moves: Vec<Position> = vec![];          // Valid positions this piece can move to including captures
  let mut attacks: Vec<Position> = vec![];              // Valid positions this piece has under attack
  let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
  let mut pins: Vec<Position> = vec![];                 // Opposing pieces pinned to the king
  let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible
  
  let is_white = board[origin.row][origin.column].as_ref().unwrap().is_white();

  let row = origin.row as i8;
  let column = origin.column as i8;
  
  // Check down
  examine_line((-1, 0), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check up
  examine_line((1, 0), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check left
  examine_line((0, -1), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check right
  examine_line((0, 1), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);
  
  // Check up-left
  examine_line((1, -1), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check up-right
  examine_line((1, 1), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check down-left
  examine_line((-1, -1), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check down-right
  examine_line((-1, 1), row, column, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  return MoveData {
    position: origin.clone(),
    valid_moves,
    attacks,
    defends,
    pins,
    checking_path
  }
}


#[cfg(test)]
mod queen_tests {
  use crate::{config::{PieceConfig, self}, board::Board, position::Position, pieces::queen::*};

  /**
   * Testing the attacks, defends and pins have all been calculated correctly through the get_queen_move_data function
   */
  #[test]
  fn test_attack_defend_pin() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 3, row: 1},
        PieceConfig {piece: String::from("queen"), white: true, column: 5, row: 3},
        PieceConfig {piece: String::from("pawn"), white: false, column: 4, row: 4},
        PieceConfig {piece: String::from("pawn"), white: false, column: 6, row: 2},
        PieceConfig {piece: String::from("king"), white: false, column: 3, row: 5}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 3, column: 5};
    let move_data = get_queen_move_data(&pos, &current_board);

    assert_eq!(move_data.position, Position {row: 3, column: 5});

    assert_eq!(move_data.attacks.len(), 19);
    assert!(move_data.attacks.contains(&Position {row: 0, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 5, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 6, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 7, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 2}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 6}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 7}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 6}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 6}));
    assert!(move_data.attacks.contains(&Position {row: 5, column: 7}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 1, column: 3}));

    assert_eq!(move_data.pins.len(), 1);
    assert!(move_data.pins.contains(&Position {row: 4, column: 4}));

    assert!(move_data.checking_path.is_none());
  }

  /**
   * Testing the checking path is calculated correctly through the get_queen_move_data function
   */
  #[test]
  fn test_check_path() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("queen"), white: true, column: 0, row: 0},
        PieceConfig {piece: String::from("king"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let pos = Position {row: 0, column: 0};
    let move_data = get_queen_move_data(&pos, &current_board);

    assert!(move_data.checking_path.is_some());

    let checking_path = move_data.checking_path.unwrap();

    assert_eq!(checking_path.len(), 6);
    assert!(checking_path.contains(&Position {row: 1, column: 1}));
    assert!(checking_path.contains(&Position {row: 2, column: 2}));
    assert!(checking_path.contains(&Position {row: 3, column: 3}));
    assert!(checking_path.contains(&Position {row: 4, column: 4}));
    assert!(checking_path.contains(&Position {row: 5, column: 5}));
    assert!(checking_path.contains(&Position {row: 6, column: 6}));
  }
}