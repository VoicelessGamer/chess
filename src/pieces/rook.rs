use crate::{
  pieces::piece::Piece, 
  position::{Position},
  move_data::MoveData,
  pieces::chess_piece::ChessPiece
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
    let mut attacks: Vec<Position> = vec![];              // Opposing pieces under attack by this piece
    let mut defends: Vec<Position> = vec![];              // Friendly pieces defended by this piece
    let mut pins: Vec<Position> = vec![];                 // Opposing pieces pinned to the king
    let mut checking_path: Option<Vec<Position>> = None;  // Path taken to attack the opposing king, if possible
    let mut current_path: Vec<Position> = vec![];

    let mut pinned: Option<Position> = None;
    let mut checking = false;
    
    // Check down
    for row in (0..origin.row).rev() {
      if examine_position(row, origin.column, board, self.white, &mut attacks, &mut defends, &mut current_path, &mut pinned, &mut checking) {
        break;
      }
    }

    if pinned.is_some() {
      pins.push(pinned.unwrap());
      pinned = None;
    }

    if checking {
      checking_path = Some(current_path);
    }
    checking = false;
    current_path = vec![];

    // Check up
    for row in origin.row+1..board.len() {
      if examine_position(row, origin.column, board, self.white, &mut attacks, &mut defends, &mut current_path, &mut pinned, &mut checking) {
        break;
      }
    }

    if pinned.is_some() {
      pins.push(pinned.unwrap());
      pinned = None;
    }

    if checking {
      checking_path = Some(current_path);
    }
    checking = false;
    current_path = vec![];

    // Check left
    for column in (0..origin.column).rev() {
      if examine_position(origin.row, column, board, self.white, &mut attacks, &mut defends, &mut current_path, &mut pinned, &mut checking) {
        break;
      }
    }

    if pinned.is_some() {
      pins.push(pinned.unwrap());
      pinned = None;
    }

    if checking {
      checking_path = Some(current_path);
    }
    checking = false;
    current_path = vec![];

    // Check right
    for column in origin.column+1..board[origin.row].len() {
      if examine_position(origin.row, column, board, self.white, &mut attacks, &mut defends, &mut current_path, &mut pinned, &mut checking) {
        break;
      }
    }

    if pinned.is_some() {
      pins.push(pinned.unwrap());
    }

    if checking {
      checking_path = Some(current_path);
    }

    return MoveData {
      position: origin,
      attacks,
      defends,
      pins,
      checking_path
    }
  }
}

fn examine_position(row:usize, column: usize, board: &Vec<Vec<Option<Box<ChessPiece>>>>, is_white: bool,
                    attacks: &mut Vec<Position>, defends: &mut Vec<Position>, current_path: &mut Vec<Position>,
                    pinned: &mut Option<Position>, checking: &mut bool) -> bool {

  match &board[row][column] {
    None => {
      if pinned.is_none() {
        attacks.push(Position {row, column});
        current_path.push(Position {row, column});
      }
      return false;
    },
    Some(chess_piece) => {
      if is_white == chess_piece.is_white() {
        if pinned.is_none() {
          defends.push(Position {row, column});
        }
        return true;
      } else {
        if pinned.is_none() {
          attacks.push(Position {row, column});
          if chess_piece.is_king() {
            *checking = true;
            return true;
          } else {
            *pinned = Some(Position {row, column});
          }
        } else {
          if !chess_piece.is_king() {
            *pinned = None;
            return true;
          }
        }
      }
      return false;
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

    assert_eq!(move_data.attacks.len(), 8);
    assert!(move_data.attacks.contains(&Position {row: 0, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 1, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 3}));
    assert!(move_data.attacks.contains(&Position {row: 3, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 4, column: 4}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 5}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 6}));
    assert!(move_data.attacks.contains(&Position {row: 2, column: 7}));

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