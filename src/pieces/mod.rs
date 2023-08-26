pub mod piece;

pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

mod piece_util;

use crate::{position::Position, piece_move::PieceMove, move_data::MoveData};
use self::piece::Piece;

/**
 * Get the relevant move data based on the Piece type in the given position
 */
pub fn get_move_data(position: &Position, board: &Vec<Vec<Option<Piece>>>, last_move: &Option<PieceMove>) -> MoveData {
  match &board[position.row][position.column] {
    Some(piece) => {
      match piece {
        Piece::Bishop(_) => bishop::get_bishop_move_data(position, board),
        Piece::Knight(_) => knight::get_knight_move_data(position, board),
        Piece::Pawn(_) => pawn::get_pawn_move_data(position, board, last_move),
        Piece::Queen(_) => queen::get_queen_move_data(position, board),
        Piece::Rook(_) => rook::get_rook_move_data(position, board),
        Piece::King(_) => king::get_king_move_data(position, board)
      }
    },
    None => todo!(),
  }
}