use crate::game::Game;
use crate::board::Board;

mod game;
mod board;

mod pieces;

fn main() {
  println!("Welcome to Chess!");

  let mut game = Game::new();

  game.play_move();
}
