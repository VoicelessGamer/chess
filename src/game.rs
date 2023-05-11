use crate::board::Board;
use crate::pieces::piece::Piece;
use crate::pieces::*;

pub struct Game<'a> {
  board: Board<'a>,
  pieces: Vec<Box<dyn Piece>>,
  white_castle: bool, // Whether white can still castle
  black_castle: bool, // Whether black can still castle
  white_player: u8, // TODO: Come back to this once player classes written
  black_player: u8, // TODO: Come back to this once player classes written
  white_turn: bool // true if it is currently white's turn
}

impl<'a> Game<'a> {
  /**
   * Initialises a board with the given dimensions. Each position is initialised to Option.None
   */
  pub fn new() -> Self {  
    let mut pieces: Vec<Box<dyn Piece>>  = vec![Box::new(king::King::new(false))]; // TODO: Pieces in a vec will not be useful, just using as a test

    Self { 
      board: Board::<'a>::new(8, 8),
      pieces,
      white_castle: true,
      black_castle: true,
      white_player: 0,
      black_player: 1,
      white_turn: true
    }
  }

  pub fn play_move(&'a mut self) {
    self.board.assign_piece(self.pieces[0].as_ref(), 3, 2);
    
    self.board.print_board();

    self.board.move_piece(3, 2, 4, 2);

    println!("#########################");
    self.board.print_board();
  }
}