pub mod piece_util {
  use crate::{
    position::Position, 
    pieces::piece::Piece
  };

  /**
   * Examines each position in a given direction from an origin point, calculating the relevant data for a MoveData struct.
   * The vectors passed into the function are updated with the calculated information.
   */
  pub fn examine_line(direction: (i8, i8), origin_row: i8, origin_column: i8, board: &Vec<Vec<Option<Piece>>>, is_white: bool, 
                      moves: &mut Vec<Position>, defends: &mut Vec<Position>, pins: &mut Vec<Position>, checking_path: &mut Option<Vec<Position>>) {

    let mut current_path: Vec<Position> = vec![];
    let mut pinned: Option<Position> = None;
    let mut verified_pin = false;
    let mut checking = false;
    
    let mut row = origin_row + direction.0;
    let mut column = origin_column + direction.1;

    /* 
     * Iterates positions in a given line, examining them, until the position is out of bounds or 
     * the examined position results in no more positions needing to be checked
     */
    while row >= 0 && column >= 0 && (row as usize) < board.len() && (column as usize) < board[row as usize].len() {
      if examine_pinnable_position(row as usize, column as usize, board, is_white, moves, defends, &mut current_path, &mut pinned, &mut verified_pin, &mut checking) {
        break;
      }
      row = row + direction.0 ;
      column = column + direction.1;
    }

    // Update the pinned position vector if a piece has been found to be pinned to the king
    if verified_pin {
      pins.push(pinned.unwrap());
    }

    // If checking is true then this line has lead to the opposing king
    if checking {
      *checking_path = Some(current_path);
    }
  }

  /**
   * Examines a position on the board and updates the reference vectors accordingly.
   * Returns true if the examined position is also a stopping point, i.e. the position contains a piece
   * of the same colour, the position contains thee opposing king or an opposing piece is in the position
   * but the pinned option is already Some().
   */
  fn examine_pinnable_position(row:usize, column: usize, board: &Vec<Vec<Option<Piece>>>, is_white: bool,
                          moves: &mut Vec<Position>, defends: &mut Vec<Position>, current_path: &mut Vec<Position>,
                          pinned: &mut Option<Position>, verified_pin: &mut bool, checking: &mut bool) -> bool {

    match &board[row][column] {
      None => {
        /*
        * If the current position doesn't contain a piece and the pinned parameter is None
        * then update the vector of positions this piece can move to and the current path
        */
        if pinned.is_none() {
          moves.push(Position {row, column});
          current_path.push(Position {row, column});
        }
        return false;
      },
      Some(chess_piece) => {
        if is_white == chess_piece.is_white() { // Piece in this position is friendly
          // If pinned is Some() the this friendly piece is behind an opposing piece on the line so it is not defended
          if pinned.is_none() {
            defends.push(Position {row, column});
          }
          return true;
        } else { // Piece in this position is an enemy piece        
          // If pinned is None then this is the first opposing piece in the line
          if pinned.is_none() {
            moves.push(Position {row, column});
            if chess_piece.is_king() {
              *checking = true;
              return true;
            } else {
              *pinned = Some(Position {row, column});
            }
          } else {
            // Reached a second opposing piece in the line
            // If this second piece is not the opposing king then it's not pinned (specifically to the king)
            if !chess_piece.is_king() {
              *pinned = None;
            }
            *verified_pin = true;
            return true;
          }
        }
        return false;
      }
    }
  }

  /**
   * Examines a position on the board and updates the reference vectors accordingly.
   */
  pub fn examine_position(row_to_check: i8, column_to_check: i8, board: &Vec<Vec<Option<Piece>>>,
                      is_white: bool, moves: &mut Vec<Position>, defends: &mut Vec<Position>, checking: &mut bool) {

    let row= row_to_check as usize;
    let column= column_to_check as usize;

    if row_to_check < 0 || column_to_check < 0 || row >= board.len() || column >= board[row].len() {
      return;
    }

    match &board[row][column] {
      None => {
        moves.push(Position {row, column});
      },
      Some(chess_piece) => {
        if is_white == chess_piece.is_white() { // Piece in this position is friendly
          defends.push(Position {row, column});
        } else { // Piece in this position is an enemy piece        
          moves.push(Position {row, column});
          if chess_piece.is_king() {
            *checking = true;
          }
        }
      }
    }
  }
}