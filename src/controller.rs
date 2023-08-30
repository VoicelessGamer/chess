use crate::{piece_move::PieceMove, config::GameConfig, game::{GameStateResult, Game}};

pub struct Controller {
  game: Game
}

impl Controller {
  pub fn new(game_config: GameConfig) -> Self {
    // TODO: Add validation of the game config, checking for position boundaries single king per side
    Self {
      game: Game::new(game_config)
    }
  }
  
  /**
   * This function initialises the game and retrieves the initial game state
   */
  pub fn initialise_game(&mut self) -> Result<GameStateResult, &'static str> {
    return self.game.initialise_game_state();
  }

  /**
   * This function should validate and process the next move for the active player
   */
  pub fn process_move(&mut self, piece_move: PieceMove) -> Result<GameStateResult, &'static str> {
    if self.game.validate_move(&piece_move) {
      return self.game.perform_move(piece_move);
    }
    Err("Move validation failed.")
  }
}