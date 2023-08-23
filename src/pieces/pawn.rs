use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData, 
  piece_move::PieceMove
};

pub fn get_pawn_move_data(origin: &Position, board: &Vec<Vec<Option<Piece>>>, last_move: &Option<PieceMove>) -> MoveData {
  let mut valid_moves: Vec<Position> = vec![];          // Valid positions this piece can move to including captures
  let mut attacks: Vec<Position> = vec![];              // Valid positions this piece has under attack
  let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
  let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible
  let mut checking = false;

  let is_white = board[origin.row][origin.column].as_ref().unwrap().is_white();

  let row = origin.row as i8; // TODO: don't need to cast
  let column = origin.column as i8;

  // Pawn attack direction is dependent on piece colour
  if is_white {
    // Check pawn move positions
    let can_move = examine_move_position(row + 1, column, board, &mut valid_moves);
    if can_move && row == 1 { // Second rank
      examine_move_position(row + 2, column, board, &mut valid_moves);
    }

    examine_attack_position(row + 1, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
    examine_attack_position(row + 1, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);

    // Check for en passant rule
    if last_move.is_some() {
      let l_move = last_move.as_ref().unwrap();
      let last_target = &l_move.target;
      let last_move_piece = board[l_move.target.row][last_target.column].as_ref().unwrap();
      // Last piece moved Black, both last piece and this piece on same row, this pawn is on the 5th Rank
      if !last_move_piece.is_white() && last_target.row == origin.row && origin.row == 4 { 
        match last_move_piece { // Check the last moved piece was a pawn
          Piece::Pawn(_) => {
            // Checking if pawn moved 2 spaces
            // Checking this here because black pawn can only move down the board, 
            // meaning current - target is guaranteed a positive value, abiding by the usize requirement
            if &l_move.current.row - last_target.row == 2 {
              // Check the 2 pawns are on adjacent columns
              if last_target.column > origin.column && last_target.column - origin.column == 1 {                
                attacks.push(Position {row: origin.row + 1, column: origin.column + 1});
              } else if origin.column > last_target.column && origin.column - last_target.column == 1 {
                attacks.push(Position {row: origin.row + 1, column: origin.column - 1});
              }
            }
          },
          _ => {} // Not en passant
        }
      }
    }
  } else {
    // Check pawn move positions
    let can_move = examine_move_position(row - 1, column, board, &mut valid_moves);
    if can_move && row == 6 { // Second last rank
      examine_move_position(row - 2, column, board, &mut valid_moves);
    }

    examine_attack_position(row - 1, column - 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);
    examine_attack_position(row - 1, column + 1, board, is_white, &mut valid_moves, &mut attacks, &mut defends, &mut checking);

    // Check for en passant rule
    if last_move.is_some() {
      let l_move = last_move.as_ref().unwrap();
      let last_target = &l_move.target;
      let last_move_piece = board[l_move.target.row][last_target.column].as_ref().unwrap();
      // Last piece moved White, both last piece and this piece on same row, this pawn is on the 4th Rank
      if last_move_piece.is_white() && last_target.row == origin.row && origin.row == 3 { 
        match last_move_piece { // Check the last moved piece was a pawn
          Piece::Pawn(_) => {
            // Checking if pawn moved 2 spaces
            // Checking this here because white pawn can only move up the board, 
            // meaning target - current is guaranteed a positive value, abiding by the usize requirement
            if last_target.row - &l_move.current.row == 2 {
              // Check the 2 pawns are on adjacent columns
              if last_target.column > origin.column && last_target.column - origin.column == 1 {                
                attacks.push(Position {row: origin.row - 1, column: origin.column + 1});
              } else if origin.column > last_target.column && origin.column - last_target.column == 1 {
                attacks.push(Position {row: origin.row - 1, column: origin.column - 1});
              }
            }
          },
          _ => {} // Not en passant
        }
      }
    }
  }

  if checking {
    checking_path = Some(vec![]);
  }

  return MoveData {
    position: origin.clone(),
    valid_moves,
    attacks,
    defends,
    pins: vec![], // Pawns cannot pin
    checking_path
  }
}

/**
 * Examines a move-only position on the board and updates the moves reference vectors accordingly.
 */
pub fn examine_move_position(row_to_check: i8, column_to_check: i8, board: &Vec<Vec<Option<Piece>>>, moves: &mut Vec<Position>) -> bool {
  let row= row_to_check as usize;
  let column= column_to_check as usize;

  if row_to_check < 0 || column_to_check < 0 || row >= board.len() || column >= board[row].len() {
    return false;
  }

  if board[row][column].is_none() {
    moves.push(Position {row, column});
    return true;
  }

  return false;
}/**
 * Examines an attack-only position on the board and updates the moves reference vectors accordingly.
 */
pub fn examine_attack_position(row_to_check: i8, column_to_check: i8, board: &Vec<Vec<Option<Piece>>>, is_white: bool,
                      valid_moves: &mut Vec<Position>, attacks: &mut Vec<Position>, defends: &mut Vec<Position>, checking: &mut bool) {

    let row= row_to_check as usize;
    let column= column_to_check as usize;

    if row_to_check < 0 || column_to_check < 0 || row >= board.len() || column >= board[row].len() {
      return;
    }

    match &board[row][column] {
      None => {
        attacks.push(Position {row, column});
      },
      Some(chess_piece) => {
        if is_white == chess_piece.is_white() { // Piece in this position is friendly
          defends.push(Position {row, column});
        } else { // Piece in this position is an enemy piece
          valid_moves.push(Position {row, column});
          attacks.push(Position {row, column});
          if chess_piece.is_king() {
            *checking = true;
          }
        }
      }
    }
}

#[cfg(test)]
mod pawn_tests {
  use crate::{config::{PieceConfig, self}, board::Board, position::Position, pieces::pawn::*};

  #[test]
  fn test_standard_positions() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 1},
        PieceConfig {piece: String::from("pawn"), white: true, column: 5, row: 1},
        PieceConfig {piece: String::from("pawn"), white: false, column: 5, row: 6},
        PieceConfig {piece: String::from("pawn"), white: false, column: 1, row: 2},
        PieceConfig {piece: String::from("pawn"), white: false, column: 5, row: 3}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    // Check white attack direction
    let pos = Position {row: 1, column: 1};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert_eq!(move_data.valid_moves.len(), 0);

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 2, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 2}));

    let pos = Position {row: 1, column: 5};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert_eq!(move_data.valid_moves.len(), 1);
    assert!(move_data.valid_moves.contains(&Position {row: 2, column: 5}));

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 2, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 6}));

    // Check black attack direction
    let pos = Position {row: 2, column: 1};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert_eq!(move_data.valid_moves.len(), 0);

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 1, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 2}));

    let pos = Position {row: 6, column: 5};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert_eq!(move_data.valid_moves.len(), 2);
    assert!(move_data.valid_moves.contains(&Position {row: 5, column: 5}));
    assert!(move_data.valid_moves.contains(&Position {row: 4, column: 5}));

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 5, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 5, column: 6}));
  }

  #[test]
  fn test_attack_defend() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 2},
        PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 1},
        PieceConfig {piece: String::from("pawn"), white: false, column: 2, row: 2},
        PieceConfig {piece: String::from("pawn"), white: false, column: 3, row: 1}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    // Check white attack direction
    let pos = Position {row: 1, column: 1};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert_eq!(move_data.position, Position {row: 1, column: 1});

    assert_eq!(move_data.valid_moves.len(), 3);
    assert!(move_data.valid_moves.contains(&Position {row: 3, column: 1}));
    assert!(move_data.valid_moves.contains(&Position {row: 2, column: 1}));
    assert!(move_data.valid_moves.contains(&Position {row: 2, column: 2}));

    assert_eq!(move_data.attacks.len(), 1);
    assert!(move_data.attacks.contains(&Position {row: 2, column: 2}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 2, column: 0}));

    assert!(move_data.pins.is_empty());

    assert!(move_data.checking_path.is_none());

    // Check black attack direction
    let pos = Position {row: 2, column: 2};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert_eq!(move_data.position, Position {row: 2, column: 2});

    assert_eq!(move_data.attacks.len(), 1);
    assert!(move_data.attacks.contains(&Position {row: 1, column: 1}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 1, column: 3}));

    assert!(move_data.pins.is_empty());

    assert!(move_data.checking_path.is_none());
  }

  #[test]
  fn test_check_path() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 1},
        PieceConfig {piece: String::from("king"), white: true, column: 3, row: 1},
        PieceConfig {piece: String::from("pawn"), white: false, column: 2, row: 2},
        PieceConfig {piece: String::from("king"), white: false, column: 0, row: 2}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    // Check white attack direction
    let pos = Position {row: 1, column: 1};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);
    
    assert!(move_data.checking_path.is_some());

    assert!(move_data.checking_path.unwrap().is_empty());

    // Check black attack direction
    let pos = Position {row: 2, column: 2};
    let move_data = get_pawn_move_data(&pos, &current_board, &None);

    assert!(move_data.checking_path.is_some());

    assert!(move_data.checking_path.unwrap().is_empty());
  }
}
