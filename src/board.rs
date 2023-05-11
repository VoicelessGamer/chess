use crate::pieces::piece::Piece;

pub struct Board<'a> {
  board: Vec<Vec<Option<&'a dyn Piece>>>
}

impl<'a> Board<'a>{
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new(rows: usize, columns: usize) -> Self {
    let board = vec![vec![None; columns]; rows];
    Self { board }
  }

  /**
   * Function call to place a given piece at a given position
   */
  pub fn assign_piece(&mut self, piece: &'a dyn Piece, x: usize, y: usize) {
    self.board[x][y] = Some(piece);
  }

  /**
   * Function call to place a given piece at a given position
   */
  pub fn move_piece(&mut self, x: usize, y: usize, new_x: usize, new_y: usize) {
    self.board[new_x][new_y] = self.board[x][y];
    self.board[x][y] = None;
  }

  /**
   * Temporarily placed function here until a console interface is in place.
   */
  pub fn print_board(&self) {
    for row in self.board.iter().rev() {
      for col in row.iter() {
        if col.is_some() {
          print!("{} ", col.unwrap().abbreviation());
        } else {
          print!("- ");
        }        
      }
      println!();
    }
  }
}