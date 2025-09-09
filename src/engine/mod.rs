use core::f32;

use crate::api_types::GameState;
pub use board::{Board, Snake};
pub use direction::Direction;
use strum::IntoEnumIterator;
pub use vector::Vector;

mod board;
mod direction;
mod vector;

pub fn get_move(game_state: &GameState) -> Direction {
    info!("Requested move");

    let board = Board::from_game_state(game_state);

    let decision = evaluate(&board, 5);

    info!("Decided on move {0} with eval {1}", decision.1, decision.0);

    return decision.1;
}

fn evaluate(board: &Board, depth: i32) -> (f32, Direction) {
    if let Some(e) = override_evaluate(board) {
        return (e, Direction::Up);
    }

    let mut enemy_moves: Vec<Vec<Direction>> = vec![];

    for enemy_snake in board.snakes.iter().skip(1) {
        let back = get_snake_backwards(enemy_snake);
        let moves: Vec<Direction> = Direction::iter().filter(|d| d != &back).collect();
        enemy_moves.push(moves);
    }

    let player_back = get_snake_backwards(&board.snakes[0]);

    let mut most = (-f32::INFINITY, Direction::Up);

    for dir in Direction::iter().filter(|d| d != &player_back) {
        let mut least = f32::INFINITY;

        for i in 0..3_u32.pow(enemy_moves.len() as u32) {
            let mut enemy_move = vec![];

            for j in 0..enemy_moves.len() {
                enemy_move.push(enemy_moves[j][(i as u32 / (3_u32.pow(j as u32)) % 3) as usize]);
            }

            let mut next_board = board.clone();

            let moves: Vec<Direction> = std::iter::once(dir)
                .chain(enemy_move.iter().map(|&d| d))
                .collect();

            next_board.iterate(&moves);

            let eval = if depth == 0 {
                base_evaluate(&next_board)
            } else {
                evaluate(&next_board, depth - 1).0 + base_evaluate(&next_board)
            };

            if eval < least {
                least = eval;
            }
        }

        if least > most.0 {
            most = (least, dir);
        }
    }

    return most;
}

fn get_snake_backwards(snake: &Snake) -> Direction {
    let vec_back = snake.body[1] - snake.body[0];

    if let Some(d) = Direction::from_vector(&vec_back) {
        d
    } else {
        // error!("Backwards vector {vec_back:?} is not convertable to a direction");
        Direction::Up
    }
}

fn base_evaluate(board: &Board) -> f32 {
    if let Some(e) = override_evaluate(board) {
        return e;
    }

    let mut eval = 0.0;

    eval -= board.snakes.len() as f32 * 10.0;

    let player_snake = &board.snakes[0];

    eval += player_snake.health as f32 * 0.01;

    return eval;
}

fn override_evaluate(board: &Board) -> Option<f32> {
    if board.player_dead || board.snakes.len() == 0 {
        return Some(-999999.0);
    }

    if board.snakes.len() == 1 {
        return Some(999999.0);
    }

    None
}
