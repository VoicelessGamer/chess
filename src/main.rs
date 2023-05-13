mod console_controller;
mod player;
mod human_player;
mod computer_player;
mod game;
mod board;
mod config;
mod pieces;
mod position;

use crate::console_controller::ConsoleController;
use crate::human_player::HumanPlayer;
use crate::game::Game;
use crate::config::*;

fn main() {
  println!("Welcome to Chess!");
  println!("");

  let mut console_controller = ConsoleController::new(
    Game::new(test_default_config()),
    Box::new(HumanPlayer {white: true}),
    //Box::new(ComputerPlayer {white: false, difficulty: 1})
    Box::new(HumanPlayer {white: true})
  );

  console_controller.run();
}

/**
 * Test function. This will be moved at a later date to come from config files
 */
fn test_default_config() -> config::GameConfig {
  let board_config = config::BoardConfig {
    pieces: vec![
      PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 1, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 2, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 3, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 4, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 5, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 6, row: 1},
      PieceConfig {piece: String::from("pawn"), white: true, column: 7, row: 1},
      PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 0},
      PieceConfig {piece: String::from("knight"), white: true, column: 1, row: 0},
      PieceConfig {piece: String::from("bishop"), white: true, column: 2, row: 0},
      PieceConfig {piece: String::from("queen"), white: true, column: 3, row: 0},
      PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
      PieceConfig {piece: String::from("bishop"), white: true, column: 5, row: 0},
      PieceConfig {piece: String::from("knight"), white: true, column: 6, row: 0},
      PieceConfig {piece: String::from("rook"), white: true, column: 7, row: 0},

      PieceConfig {piece: String::from("pawn"), white: false, column: 0, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 1, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 2, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 3, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 4, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 5, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 6, row: 6},
      PieceConfig {piece: String::from("pawn"), white: false, column: 7, row: 6},
      PieceConfig {piece: String::from("rook"), white: false, column: 0, row: 7},
      PieceConfig {piece: String::from("knight"), white: false, column: 1, row: 7},
      PieceConfig {piece: String::from("bishop"), white: false, column: 2, row: 7},
      PieceConfig {piece: String::from("queen"), white: false, column: 3, row: 7},
      PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7},
      PieceConfig {piece: String::from("bishop"), white: false, column: 5, row: 7},
      PieceConfig {piece: String::from("knight"), white: false, column: 6, row: 7},
      PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7},
    ],
    rows: 8,
    columns: 8
  };

  GameConfig {
    initial_board: board_config,
    white_castle: true,
    black_castle: true,
    white_turn: true,
  }
}
