mod controller;
mod io_controller;
mod view;
mod io_view;
mod game;
mod board;
mod config;
mod pieces;
mod player_move;
mod position;

use crate::io_controller::IOController;
use crate::io_view::IOView;
use crate::game::Game;
use crate::config::*;

fn main() {
  println!("Welcome to Chess!");
  println!("");

  let mut game = Game::new(
    IOController::new(true, true),
    IOView {use_unicode: true},
    test_default_config()
  );

  game.run();
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
