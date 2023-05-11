mod game;
mod board;
mod config;
mod pieces;

use crate::game::Game;
use crate::board::Board;
use crate::config::*;

fn main() {
  println!("Welcome to Chess!");

  let mut game = Game::new(test_default_config());

  game.play_move();
}

/**
 * Test function. This will be moved at a later date to come from config files
 */
fn test_default_config() -> config::GameConfig {
  let board_config = config::BoardConfig {
    pieces: vec![
      PieceConfig {piece: String::from("pawn"), white: true, x: 0, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 1, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 2, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 3, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 4, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 5, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 6, y: 1},
      PieceConfig {piece: String::from("pawn"), white: true, x: 7, y: 1},
      PieceConfig {piece: String::from("rook"), white: true, x: 0, y: 0},
      PieceConfig {piece: String::from("knight"), white: true, x: 1, y: 0},
      PieceConfig {piece: String::from("bishop"), white: true, x: 2, y: 0},
      PieceConfig {piece: String::from("queen"), white: true, x: 3, y: 0},
      PieceConfig {piece: String::from("king"), white: true, x: 4, y: 0},
      PieceConfig {piece: String::from("bishop"), white: true, x: 5, y: 0},
      PieceConfig {piece: String::from("knight"), white: true, x: 6, y: 0},
      PieceConfig {piece: String::from("rook"), white: true, x: 7, y: 0},

      PieceConfig {piece: String::from("pawn"), white: false, x: 0, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 1, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 2, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 3, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 4, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 5, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 6, y: 6},
      PieceConfig {piece: String::from("pawn"), white: false, x: 7, y: 6},
      PieceConfig {piece: String::from("rook"), white: false, x: 0, y: 7},
      PieceConfig {piece: String::from("knight"), white: false, x: 1, y: 7},
      PieceConfig {piece: String::from("bishop"), white: false, x: 2, y: 7},
      PieceConfig {piece: String::from("queen"), white: false, x: 3, y: 7},
      PieceConfig {piece: String::from("king"), white: false, x: 4, y: 7},
      PieceConfig {piece: String::from("bishop"), white: false, x: 5, y: 7},
      PieceConfig {piece: String::from("knight"), white: false, x: 6, y: 7},
      PieceConfig {piece: String::from("rook"), white: false, x: 7, y: 7},
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
