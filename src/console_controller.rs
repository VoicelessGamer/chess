use crate::{
  game::Game, 
  pieces::chess_piece::ChessPiece,
  player::Player
};

pub struct ConsoleController<WP: Player, BP: Player> {
  game: Game,
  white_player: Box<WP>,
  black_player: Box<BP>
}

impl<WP: Player, BP: Player> ConsoleController<WP, BP> {
  pub fn new(game: Game, white_player: Box<WP>, black_player: Box<BP>) -> Self {
    Self { 
      game,
      white_player,
      black_player
    }
  }

  pub fn run(&mut self) {
    self.initial_display();

    println!("");
    println!("#########################");
    println!("");
    
    while self.game.is_incomplete() {
      let current_board;

      if self.game.is_white_turn() {
        let player_move = self.white_player.get_move();
        current_board = self.game.play_move(player_move.0, player_move.1);
      } else {
        current_board = self.game.get_current_board();
      }
      print_board(current_board);
    }
  }


  pub fn initial_display(&mut self) {
    print_board(self.game.get_current_board());
  }
}


/**
 * Prints the current state of the board to the console output
 */
pub fn print_board(board: &Vec<Vec<Option<Box<ChessPiece>>>>) {
  for row in board.iter().rev() {
    for col in row.iter() {
      if col.is_some() {
        let piece = col.as_ref().unwrap();
        match &**piece {
            ChessPiece::Bishop(bishop) => {
              if bishop.white { print!("w") } else { print!("b") };
              print!("B ");
            },
            ChessPiece::King(king) => {
              if king.white { print!("w") } else { print!("b") };
              print!("K ");
            },
            ChessPiece::Knight(knight) => {
              if knight.white { print!("w") } else { print!("b") };
              print!("N ");
            },
            ChessPiece::Pawn(pawn) => {
              if pawn.white { print!("w") } else { print!("b") };
              print!("P ");
            },
            ChessPiece::Queen(queen) => {
              if queen.white { print!("w") } else { print!("b") };
              print!("Q ");
            },
            ChessPiece::Rook(rook) => {
              if rook.white { print!("w") } else { print!("b") };
              print!("R ");
            },
        }
        
      } else {
        print!("-- ");
      }        
    }
    println!();
  }
}