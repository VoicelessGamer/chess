use chess::{game::{Game, GameState}, config::{self, PieceConfig}, piece_move::PieceMove, position::Position};

mod common;

/**
 * Tests a full game run through with the scholars mate checkmate result for white
 */
#[test]
fn test_checkmate_scholars_mate() {
  let game_config = config::GameConfig {
    initial_board: config::BoardConfig {
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
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 7}
      ],
      rows: 8,
      columns: 8
    },
    white_long_castle: true,
    white_short_castle: true,
    black_long_castle: true,
    black_short_castle: true,
    white_turn: true
  };

  // pawn e4   (w)
  // pawn e5   (b)
  // bishop c4 (w)
  // kngiht c6 (b)
  // queen h5  (w)
  // knight f6 (b)
  // queen f7  (w) *checkmate*

  let moves = vec![
    PieceMove { start: Position{ row: 1, column: 4 }, end: Position{ row: 3, column: 4 }, promotion: None},
    PieceMove { start: Position{ row: 6, column: 4 }, end: Position{ row: 4, column: 4 }, promotion: None},
    PieceMove { start: Position{ row: 0, column: 5 }, end: Position{ row: 3, column: 2 }, promotion: None},
    PieceMove { start: Position{ row: 7, column: 1 }, end: Position{ row: 5, column: 2 }, promotion: None},
    PieceMove { start: Position{ row: 0, column: 3 }, end: Position{ row: 4, column: 7 }, promotion: None},
    PieceMove { start: Position{ row: 7, column: 6 }, end: Position{ row: 5, column: 5 }, promotion: None},
    PieceMove { start: Position{ row: 4, column: 7 }, end: Position{ row: 6, column: 5 }, promotion: None}
  ];

  let mut game = Game::new(
    common::test_controller::TestController::new(moves),
    common::test_view::TestView {},
    game_config
  );

  // Providing the .run() call completes, it is considered a passed test
  // Test will panic if the supplied moves do not end in checkmate
  game.run();

  let mut white_win = false;
  if let GameState::WhiteWin = game.get_state().game_state {
    white_win = true;
  }
  assert!(white_win); // White won by checkmate
}


/**
 * Tests a stalemate situation where the black king has no valid moves 
 */
#[test]
fn test_stalemate() {
  let game_config = config::GameConfig {
    initial_board: config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("queen"), white: true, column: 7, row: 5},
        PieceConfig {piece: String::from("king"), white: true, column: 7, row: 0},
        PieceConfig {piece: String::from("rook"), white: true, column: 1, row: 0},
        PieceConfig {piece: String::from("king"), white: false, column: 0, row: 7},
      ],
      rows: 8,
      columns: 8
    },
    white_long_castle: true,
    white_short_castle: true,
    black_long_castle: true,
    black_short_castle: true,
    white_turn: true
  };

  let moves = vec![
    PieceMove { start: Position{ row: 5, column: 7 }, end: Position{ row: 6, column: 7 }, promotion: None}
  ];

  let mut game = Game::new(
    common::test_controller::TestController::new(moves),
    common::test_view::TestView {},
    game_config
  );

  // Providing the .run() call completes, it is considered a passed test
  // Test will panic if the supplied moves do not end in checkmate
  game.run();

  let mut stalemate = false;
  if let GameState::Stalemate = game.get_state().game_state {
    stalemate = true;
  }
  assert!(stalemate); // Game ended with stalemate
}