use crate::{
  pieces::piece::Piece, 
  position::Position,
  move_data::MoveData,
  pieces::chess_piece::ChessPiece,
  pieces::piece_util::piece_util::examine_position
};


#[derive(Clone)]
pub struct King {
  white: bool
}

impl King {
  pub fn new(white: bool) -> King {
    King { white }
  }
}

impl Piece for King {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData {
    let mut attacks: Vec<Position> = vec![]; // Opposing pieces under attack by this piece
    let mut defends: Vec<Position> = vec![]; // Friendly pieces defended by this piece

    let row = origin.row as i8;
    let column = origin.column as i8;

    // Examine each possible position for a king
    examine_position(row + 1, column, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row - 1, column, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row + 1, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row - 1, column + 1, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row - 1, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut false);
    examine_position(row + 1, column - 1, board, self.is_white(), &mut attacks, &mut defends, &mut false);

    return MoveData {
      position: origin,
      attacks,
      defends,
      pins: vec![], // Kings cannot pin
      checking_path: None // Kings cannot check
    }
  }
}

#[cfg(test)]
mod king_tests {
  use crate::{config::{PieceConfig, self}, board::Board, pieces::piece::Piece, position::Position};

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

    let move_data = current_board[3][3].as_ref().unwrap().get_move_data(Position {row: 3, column: 3}, &current_board);

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

    let move_data = current_board[0][0].as_ref().unwrap().get_move_data(Position {row: 0, column: 0}, &current_board);

    assert_eq!(move_data.position, Position {row: 0, column: 0});

    assert_eq!(move_data.attacks.len(), 2);
    assert!(move_data.attacks.contains(&Position {row: 1, column: 1}));
    assert!(move_data.attacks.contains(&Position {row: 0, column: 1}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 1, column: 0}));

    assert!(move_data.pins.is_empty());

    assert!(move_data.checking_path.is_none());
  }
}
