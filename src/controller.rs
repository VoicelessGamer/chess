use crate::{piece_move::PieceMove, config::GameConfig, game::{GameStateResult, Game}, pieces::piece::Piece};

pub const VALID_PROMOTIONS: [&str; 4] = ["B", "N", "Q", "R"];

pub struct Controller {
  game: Game
}

impl Controller {
  pub fn new(game_config: GameConfig) -> Result<Self, &'static str> {
    // TODO: Add validation of the game config, checking for position boundaries single king per side
    let mut game = Game::new(game_config);
    let game_state_result = game.initialise_game_state();
    if game_state_result.is_err() {
      return Err("Unable to initialise the game state.");
    }

    Ok(Self {
      game
    })
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
  pub fn process_move(&mut self, piece_move: PieceMove) -> Result<GameStateResult, &'static str> {
    let game_state_result = self.game.get_game_state();
    if self.validate_move(&piece_move, &game_state_result) {
      return self.game.perform_move(piece_move);
    }
    Err("Move validation failed.")
  }

  /**
   * Validate the player move against the list of calculated valid moves in the game state. 
   * Returns true if the move can be made.
   */
  fn validate_move(&mut self, piece_move: &PieceMove, game_state_result: &GameStateResult) -> bool {
    if let Some(valid_positions) = game_state_result.game_state.valid_moves.get(&piece_move.start) {
      if valid_positions.contains(&piece_move.end) {
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
    return false;
  }
}

#[cfg(test)]
mod controller_tests {
  use crate::{config::{BoardConfig, PieceConfig, GameConfig}, piece_move::PieceMove, position::Position};

  use super::Controller;

  /**
   * 
   */
  #[test]
  fn valid_standard_move() {
    let game_config = GameConfig {
      initial_board: BoardConfig {
        pieces: vec![
          PieceConfig {piece: String::from("king"), white: true, column: 4, row: 0},
          PieceConfig {piece: String::from("king"), white: false, column: 4, row: 7}
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

    let controller_result = Controller::new(game_config);
    assert!(controller_result.is_ok());

    let mut controller = controller_result.unwrap();
    let game_state_result = controller.get_game_state();

    let is_valid = controller.validate_move(
      &PieceMove { start: Position{ row: 0, column: 4 }, end: Position{ row: 0, column: 5 }, promotion: None}, 
      &game_state_result
    );

    assert!(is_valid);
  }

  /**
   * 
   */
  #[test]
  fn invalid_standard_move() {
  }

  /**
   * 
   */
  #[test]
  fn valid_promotion_move() {
  }

  /**
   * 
   */
  #[test]
  fn invalid_promotion_move() {
  }
}