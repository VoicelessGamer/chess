use crate::{config::GameConfig, game::Game, model::GameStateResult, model::PieceMove, pieces::piece::Piece};

pub const VALID_PROMOTIONS: [&str; 4] = ["B", "N", "Q", "R"];

pub struct Controller {
  game: Game
}

impl Controller {
  pub fn new(game_config: GameConfig) -> Result<Self, &'static str> {
    // TODO: Add validation of the game config, checking for position boundaries
    let mut game = Game::new(game_config);
    let game_state_result = game.initialise_game_state();
    if game_state_result.is_err() {
      return Err("Unable to initialise the game state.");
    }

    Ok(Self {game})
  }
 
  /**
   * Returns the current game state
   */
  pub fn get_game_state(&mut self) -> GameStateResult {
    return self.game.get_game_state();
  }

  /**
   * This function should validate and process the next move for the active player
   */
  pub fn process_move(&mut self, piece_move: PieceMove) -> Result<GameStateResult, String> {
    let game_state_result = self.game.get_game_state();
    if !self.validate_move_selection(&piece_move, &game_state_result) {
      return Err("Invalid move selection.".to_string());
    }

    let move_result = self.game.perform_move(piece_move.clone());

    let game_state_result = move_result.map_err(|err| {format!("Failed to perform requested move. Reason: {}", err)})?;

    return Ok(game_state_result);
  }

  /**
   * Validates the move request itself to make sure promotion requests are correct for the specified piece.
   * Note: The game will validate the move itself.
   */
  fn validate_move_selection(&mut self, piece_move: &PieceMove, game_state_result: &GameStateResult) -> bool {
    // Validate promotion move
    // Check if the piece is on the furthest or nearest rank based on piece colour
    if (game_state_result.game_state.white_turn && piece_move.end.row == 7) || (!game_state_result.game_state.white_turn && piece_move.end.row == 0) {
      // Check if piece moved was a pawn
      match game_state_result.board[piece_move.start.row][piece_move.start.column].as_ref().unwrap() {
        Piece::Pawn(_) => {
          // Promotion not supplied when it should have been or provided promotion is invalid
          if piece_move.promotion.is_none() || !VALID_PROMOTIONS.contains(&piece_move.promotion.as_ref().unwrap().as_str()){
            return false;
          }
          return true; // Valid promotion move
        },
        _ => return piece_move.promotion.is_none() // Returns false if a promotion has been provided when it's not a promotion move
      }
    }

    // Returns false if a promotion has been provided when it's not a promotion move
    return piece_move.promotion.is_none(); 
  }
}

#[cfg(test)]
mod controller_tests {
  use std::collections::HashMap;

  use crate::{config::{BoardConfig, CastlingConfig, GameConfig, PieceConfig}, model::{CastlingState, GameState, GameStateResult, PlayerState, State}, model::PieceMove, pieces::piece::Piece, model::Position};

  use super::Controller;

  /**
   * Tests the process_move function performs the provided move when supplied a valid move.
   */
  #[test]
  fn valid_standard_move_process() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("queen"), white: true, column: 0, row: 0},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let controller_result = Controller::new(game_config);
    assert!(controller_result.is_ok());

    let mut controller = controller_result.unwrap();

    let move_result = controller.process_move(
      PieceMove {start: Position{row: 0, column: 4}, end: Position{row: 1, column: 4}, promotion: None}
    );

    assert!(move_result.is_ok());

    let game_state_result = move_result.unwrap();

    assert!(game_state_result.board[0][4].is_none());
    assert!(game_state_result.board[1][4].is_some());
    assert_eq!(game_state_result.board[1][4].as_ref().unwrap(), &Piece::King(true));
  }

  /**
   * Tests the process_move function returns an error if the provided move is not a valid move.
   */
  #[test]
  fn invalid_standard_move_process() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let controller_result = Controller::new(game_config);
    assert!(controller_result.is_ok());

    let mut controller = controller_result.unwrap();

    let move_result = controller.process_move(
      PieceMove {start: Position{row: 0, column: 4}, end: Position{row: 2, column: 4}, promotion: None}
    );

    assert!(move_result.is_err());
  }

  /**
   * Tests the validate_move_selection function correctly identifies the provided move is valid pawn promotion and returns true.
   */
  #[test]
  fn valid_promotion_move_selection() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 6},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let controller_result = Controller::new(game_config);
    assert!(controller_result.is_ok());

    let mut white_moves = HashMap::new();
    white_moves.insert(Position{row: 6, column: 0}, vec![Position{row: 7, column: 0}]);

    let mut controller = controller_result.unwrap();
    let game_state_result: GameStateResult = GameStateResult {
      board: vec![
        vec![None, None, None, None, Some(Piece::King(true)), None, None, None],
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![Some(Piece::Pawn(true)), None, None, None, None, None, None, None], 
        vec![None, None, None, None, Some(Piece::King(false)), None, None, None]
      ],
      game_state: GameState {
        state: State::Active,
        white_turn: true,
        white_state: PlayerState{
            in_check: false,
            castling_state: CastlingState{long_castle: false, short_castle: false},
            valid_moves: white_moves,
            last_move: None,
        },
        black_state: PlayerState{
            in_check: false,
            castling_state: CastlingState{long_castle: false, short_castle: false},
            valid_moves: HashMap::new(),
            last_move: None,
        },
      },
    };

    let is_valid = controller.validate_move_selection(
      &PieceMove {start: Position{row: 6, column: 0}, end: Position{row: 7, column: 0}, promotion: Some("Q".to_string())},
      &game_state_result
    );

    assert!(is_valid);
  }

  /**
   * Tests the validate_move_selection function with a promotion provided on a pawn move that doesn't reach the required rank. Should return false.
   */
  #[test]
  fn invalid_promotion_move_selection() {    
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("pawn"), white: true, column: 0, row: 5},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let controller_result = Controller::new(game_config);
    assert!(controller_result.is_ok());

    let mut white_moves = HashMap::new();
    white_moves.insert(Position{row: 5, column: 0}, vec![Position{row: 6, column: 0}]);

    let mut controller = controller_result.unwrap();
    let game_state_result: GameStateResult = GameStateResult {
      board: vec![
        vec![None, None, None, None, Some(Piece::King(true)), None, None, None],
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![Some(Piece::Pawn(true)), None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, Some(Piece::King(false)), None, None, None]
      ],
      game_state: GameState {
        state: State::Active,
        white_turn: true,
        white_state: PlayerState{
            in_check: false,
            castling_state: CastlingState{long_castle: false, short_castle: false},
            valid_moves: white_moves,
            last_move: None,
        },
        black_state: PlayerState{
            in_check: false,
            castling_state: CastlingState{long_castle: false, short_castle: false},
            valid_moves: HashMap::new(),
            last_move: None,
        },
      },
    };

    let is_valid = controller.validate_move_selection(
      &PieceMove {start: Position{row: 5, column: 0}, end: Position{row: 6, column: 0}, promotion: Some("Q".to_string())},
      &game_state_result
    );

    assert!(!is_valid);
  }

  /**
   * Tests the validate_move_selection function with a promotion provided on a non-pawn move. Should return false.
   */
  #[test]
  fn invalid_piece_promotion_move_selection() {
    let game_config = GameConfig {
      board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("rook"), white: true, column: 0, row: 6},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
        ],
        rows: 8,
        columns: 8
      },
      white_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      black_castling: CastlingConfig {
        long_castle: true,
        short_castle: true
      },
      white_turn: true
    };

    let controller_result = Controller::new(game_config);
    assert!(controller_result.is_ok());

    let mut white_moves = HashMap::new();
    white_moves.insert(Position{row: 6, column: 0}, vec![Position{row: 7, column: 0}]);

    let mut controller = controller_result.unwrap();
    let game_state_result: GameStateResult = GameStateResult {
      board: vec![
        vec![None, None, None, None, Some(Piece::King(true)), None, None, None],
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None, None, None, None, None, None, None, None], 
        vec![None ,None, None, None, None, None, None, None], 
        vec![Some(Piece::Rook(true)), None, None, None, None, None, None, None], 
        vec![None, None, None, None, Some(Piece::King(false)), None, None, None]
      ],
      game_state: GameState {
        state: State::Active,
        white_turn: true,
        white_state: PlayerState{
            in_check: false,
            castling_state: CastlingState{long_castle: false, short_castle: false},
            valid_moves: white_moves,
            last_move: None,
        },
        black_state: PlayerState{
            in_check: false,
            castling_state: CastlingState{long_castle: false, short_castle: false},
            valid_moves: HashMap::new(),
            last_move: None,
        },
      },
    };

    let is_valid = controller.validate_move_selection(
      &PieceMove {start: Position{row: 6, column: 0}, end: Position{row: 7, column: 0}, promotion: Some("Q".to_string())},
      &game_state_result
    );

    assert!(!is_valid);
  }
}