use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::chess_piece::ChessPiece,
  pieces::piece_util::piece_util::examine_position
};


#[derive(Clone)]
pub struct Knight {
  white: bool
}

impl Knight {
  pub fn new(white: bool) -> Knight {
    Knight { white }
  }
}

impl Piece for Knight {
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

    // Examine each possible position for a knight
    examine_position(row + 2, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row + 1, column + 2, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row - 1, column + 2, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row - 2, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row - 2, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row - 1, column - 2, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row + 1, column - 2, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    examine_position(row + 2, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut checking);
    
    if checking {
      checking_path = Some(vec![]);
    }

    return MoveData {
      position: origin,
      attacks,
      defends,
      pins: vec![], // Knights cannot pin
      checking_path
    }
  }
}

#[cfg(test)]
mod knight_tests {
  use crate::{config::{PieceConfig, self}, board::Board, pieces::piece::Piece, position::Position};

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

    let move_data = current_board[3][3].as_ref().unwrap().get_move_data(Position {row: 3, column: 3}, &current_board);

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

    let move_data = current_board[1][2].as_ref().unwrap().get_move_data(Position {row: 1, column: 2}, &current_board);

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

    let move_data = current_board[1][0].as_ref().unwrap().get_move_data(Position {row: 1, column: 0}, &current_board);

    assert!(move_data.checking_path.is_some());

    // If checking the king there is no checking path for a knight as it cannot be blocked
    assert!(move_data.checking_path.unwrap().is_empty());
  }
}
