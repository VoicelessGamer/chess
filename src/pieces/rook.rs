use crate::{
  pieces::piece::Piece, 
  position::{Position},
  move_data::MoveData,
  pieces::chess_piece::ChessPiece,
  pieces::move_data_util::move_data_util
};

#[derive(Clone)]
pub struct Rook {
  white: bool
}

impl Rook {
  pub fn new(white: bool) -> Rook {
    Rook { white }
  }
}

impl Piece for Rook {
  fn is_white(&self) -> bool {
    self.white
  }

  fn get_move_data(&self, origin: Position, board: &Vec<Vec<Option<Box<ChessPiece>>>>) -> MoveData {
    let mut moves: Vec<Position> = vec![];                // Opposing pieces under attack by this piece
    let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
    let mut pins: Vec<Position> = vec![];                 // Opposing pieces pinned to the king
    let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible

    let row = origin.row as i8;
    let column = origin.column as i8;
    
    // Check down
    move_data_util::examine_line((-1, 0), row, column, board, self.white, &mut moves, &mut defends, &mut pins, &mut checking_path);

    // Check up
    move_data_util::examine_line((1, 0), row, column, board, self.white, &mut moves, &mut defends, &mut pins, &mut checking_path);

    // Check left
    move_data_util::examine_line((0, -1), row, column, board, self.white, &mut moves, &mut defends, &mut pins, &mut checking_path);

    // Check right
    move_data_util::examine_line((0, 1), row, column, board, self.white, &mut moves, &mut defends, &mut pins, &mut checking_path);

    return MoveData {
      position: origin,
      moves,
      defends,
      pins,
      checking_path
    }
  }
}

#[cfg(test)]
mod rook_tests {
    use crate::{config::{PieceConfig, self}, board::Board, pieces::piece::Piece, position::Position};

  #[test]
  fn test_attack_defend_pin() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 2},
        PieceConfig {piece: String::from("rook"), white: true, column: 4, row: 2},
        PieceConfig {piece: String::from("pawn"), white: false, column: 4, row: 4},
        PieceConfig {piece: String::from("king"), white: false, column: 4, row: 5}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let move_data = current_board[2][4].as_ref().unwrap().get_move_data(Position {row: 2, column: 4}, &current_board);

    assert_eq!(move_data.position, Position {row: 2, column: 4});

    assert_eq!(move_data.moves.len(), 8);
    assert!(move_data.moves.contains(&Position {row: 0, column: 4}));
    assert!(move_data.moves.contains(&Position {row: 1, column: 4}));
    assert!(move_data.moves.contains(&Position {row: 2, column: 3}));
    assert!(move_data.moves.contains(&Position {row: 3, column: 4}));
    assert!(move_data.moves.contains(&Position {row: 4, column: 4}));
    assert!(move_data.moves.contains(&Position {row: 2, column: 5}));
    assert!(move_data.moves.contains(&Position {row: 2, column: 6}));
    assert!(move_data.moves.contains(&Position {row: 2, column: 7}));

    assert_eq!(move_data.defends.len(), 1);
    assert!(move_data.defends.contains(&Position {row: 2, column: 2}));

    assert_eq!(move_data.pins.len(), 1);
    assert!(move_data.pins.contains(&Position {row: 4, column: 4}));

    assert!(move_data.checking_path.is_none());
  }

  #[test]
  fn test_check_path() {
    let board_config = config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 2},
        PieceConfig {piece: String::from("king"), white: false, column: 0, row: 6}
      ],
      rows: 8,
      columns: 8
    };

    let mut board = Board::new(&board_config);
    let current_board = board.get_current_board();

    let move_data = current_board[2][0].as_ref().unwrap().get_move_data(Position {row: 2, column: 0}, &current_board);

    assert!(move_data.checking_path.is_some());

    let checking_path = move_data.checking_path.unwrap();

    assert_eq!(checking_path.len(), 3);
    assert!(checking_path.contains(&Position {row: 3, column: 0}));
    assert!(checking_path.contains(&Position {row: 4, column: 0}));
    assert!(checking_path.contains(&Position {row: 5, column: 0}));

  }
}