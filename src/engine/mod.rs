pub use board::Board;
pub use direction::Direction;
pub use vector::Vector;

mod board;
mod direction;
mod vector;

use crate::api_types::GameState;

pub fn get_move(game_state: &GameState) -> Direction {
    let board = Board::from_game_state(game_state);

    Direction::Up
}
