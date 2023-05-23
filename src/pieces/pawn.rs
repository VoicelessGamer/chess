use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::chess_piece::ChessPiece,
  pieces::piece_util::piece_util::examine_position
};

#[derive(Clone)]
pub struct Pawn {
  white: bool
}

impl Pawn {
  pub fn new(white: bool) -> Pawn {
    Pawn { white }
  }
}

impl Piece for Pawn {
  fn is_white(&self) -> bool {
    self.white
  }
  
  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData {
    let mut attacks: Vec<Position> = vec![];              // Opposing pieces under attack by this piece
    let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
    let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible
    let mut checking = false;

    let row = origin.row as i8;
    let column = origin.column as i8;

    // Pawn attack direction is dependent on piece colour
    if self.is_white() {
      examine_position(row + 1, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
      examine_position(row + 1, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    } else {
      examine_position(row - 1, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
      examine_position(row - 1, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    }

    if checking {
      checking_path = Some(vec![]);
    }

    return MoveData {
      position: origin,
      attacks,
      defends,
      pins: vec![], // Pawns cannot pin
      checking_path
    }
  }
}

#[cfg(test)]
mod pawn_tests {
  use crate::{config::{PieceConfig, self}, board::Board, pieces::piece::Piece, position::Position};

  #[test]
  fn test_standard_positions() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 1},
        PieceConfig {piece: String::from("pawn"), white: false, column: 1, row: 2}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    // Check white attack direction
    let move_data = current_board[1][1].as_ref().unwrap().get_move_data(Position {row: 1, column: 1}, &current_board);

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 2, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 2}));

    // Check black attack direction
    let move_data = current_board[2][1].as_ref().unwrap().get_move_data(Position {row: 2, column: 1}, &current_board);

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 1, column: 0}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 2}));
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
    let move_data = current_board[1][1].as_ref().unwrap().get_move_data(Position {row: 1, column: 1}, &current_board);

    assert_eq!(move_data.position, Position {row: 1, column: 1});

    assert_eq!(move_data.attacks.len(), 1);
    assert!(move_data.attacks.contains(&Position {row: 2, column: 2}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 2, column: 0}));

    assert!(move_data.pins.is_empty());

    assert!(move_data.checking_path.is_none());

    // Check black attack direction
    let move_data = current_board[2][2].as_ref().unwrap().get_move_data(Position {row: 2, column: 2}, &current_board);

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
    let move_data = current_board[1][1].as_ref().unwrap().get_move_data(Position {row: 1, column: 1}, &current_board);
    
    assert!(move_data.checking_path.is_some());

    assert!(move_data.checking_path.unwrap().is_empty());

    // Check black attack direction
    let move_data = current_board[2][2].as_ref().unwrap().get_move_data(Position {row: 2, column: 2}, &current_board);

    assert!(move_data.checking_path.is_some());

    assert!(move_data.checking_path.unwrap().is_empty());
  }
}
