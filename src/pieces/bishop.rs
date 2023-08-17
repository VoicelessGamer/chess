use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::piece_util::piece_util::examine_line
};

pub fn get_bishop_move_data(origin: Position, board: &Vec<Vec<Option<Piece>>>) -> MoveData {
  let mut attacks: Vec<Position> = vec![];              // Opposing pieces under attack by this piece
  let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
  let mut pins: Vec<Position> = vec![];                 // Opposing pieces pinned to the king
  let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible

  let is_white = board[origin.row][origin.column].as_ref().unwrap().is_white();

  let row = origin.row as i8;
  let column = origin.column as i8;

  // Check up-left
  examine_line((1, -1), row, column, board, is_white, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check up-right
  examine_line((1, 1), row, column, board, is_white, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check down-left
  examine_line((-1, -1), row, column, board, is_white, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  // Check down-right
  examine_line((-1, 1), row, column, board, is_white, &mut attacks, &mut defends, &mut pins, &mut checking_path);

  return MoveData {
    position: origin,
    attacks,
    defends,
    pins,
    checking_path
  }
}

#[cfg(test)]
mod bishop_tests {
  use crate::{config::{PieceConfig, self}, board::Board, position::Position, pieces::bishop::*};

  #[test]
  fn test_attack_defend_pin() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 3},
        PieceConfig {piece: String::from("bishop"), white: true, column: 2, row: 2},
        PieceConfig {piece: String::from("pawn"), white: false, column: 4, row: 4},
        PieceConfig {piece: String::from("king"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let move_data = get_bishop_move_data(Position {row: 2, column: 2}, &current_board);

    assert_eq!(move_data.position, Position {row: 2, column: 2});

    assert_eq!(move_data.attacks.len(), 6);
    assert!(move_data.attacks.contains(&Position {row: 0, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 0, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 4}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 3, column: 1}));

    assert_eq!(move_data.pins.len(), 1);
    assert!(move_data.pins.contains(&Position {row: 4, column: 4}));

    assert!(move_data.checking_path.is_none());
  }

  #[test]
  fn test_check_path() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("bishop"), white: true, column: 5, row: 1},
        PieceConfig {piece: String::from("king"), white: false, column: 2, row: 4}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let move_data = get_bishop_move_data(Position {row: 1, column: 5}, &current_board);

    assert!(move_data.checking_path.is_some());

    let checking_path = move_data.checking_path.unwrap();

    assert_eq!(checking_path.len(), 2);
    assert!(checking_path.contains(&Position {row: 3, column: 3}));
    assert!(checking_path.contains(&Position {row: 2, column: 4}));
  }
}