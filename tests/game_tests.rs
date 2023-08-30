use chess::{game::{Game, State}, config::{self, PieceConfig}, piece_move::PieceMove, position::Position};

/**
 * Tests a full game run through with the scholars mate checkmate result for white
 */
#[test]
fn game_state_checkmate_scholars_mate() {
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

  let mut game = Game::new(game_config);
  let mut result = game.initialise_game_state();

  assert!(result.is_ok());

  let mut iter = moves.iter();
  while let Some(piece_move) = iter.next() {
    result = game.perform_move(piece_move.to_owned());
  }

  assert!(result.is_ok());

  let game_state_result = result.unwrap();

  let mut white_win = false;
  if let State::WhiteWin = game_state_result.game_state.state {
    white_win = true;
  }
  assert!(white_win); // White won by checkmate
}

/**
 * Tests a stalemate situation where the black king has no valid moves 
 */
#[test]
fn game_state_stalemate() {
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

  let mut game = Game::new(game_config);
  let mut result = game.initialise_game_state();

  assert!(result.is_ok());

  let mut iter = moves.iter();
  while let Some(piece_move) = iter.next() {
    result = game.perform_move(piece_move.to_owned());
  }

  assert!(result.is_ok());

  let game_state_result = result.unwrap();

  let mut stalemate = false;
  if let State::Stalemate = game_state_result.game_state.state {
    stalemate = true;
  }
  assert!(stalemate); // Game ended with stalemate
}


/**
 * Tests the game is set to an error state if there are no kings in play. The logic cannot function without a king for each side. 
 */
#[test]
fn game_state_error_no_kings() {
  let game_config = config::GameConfig {
    initial_board: config::BoardConfig {
      pieces: vec![
        PieceConfig {piece: String::from("rook"), white: false, column: 7, row: 5},
        PieceConfig {piece: String::from("rook"), white: true, column: 1, row: 0}
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
    PieceMove { start: Position{ row: 0, column: 1 }, end: Position{ row: 1, column: 1 }, promotion: None}
  ];

  let mut game = Game::new(game_config);
  let mut result = game.initialise_game_state();

  assert!(result.is_ok());

  let mut iter = moves.iter();
  while let Some(piece_move) = iter.next() {
    result = game.perform_move(piece_move.to_owned());
  }

  assert!(result.is_ok());

  let game_state_result = result.unwrap();

  let mut errored = false;
  if let State::Error = game_state_result.game_state.state {
    errored = true;
  }
  assert!(errored); // Game ended with stalemate
}