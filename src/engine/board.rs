use super::vector::Vector;
use std::collections::VecDeque;

pub struct Snake {
    body: VecDeque<Vector>,
    health: i32,
}

impl Snake {
    pub fn from_battlesnake(snake: &crate::api_types::Battlesnake) -> Self {
        Snake {
            body: snake.body.clone().into(),
            health: snake.health,
        }
    }
}

pub struct Board {
    snakes: Vec<Snake>,
    food: Vec<Vector>,
    hazards: Vec<Vector>,
    size: Vector,
}

impl Board {
    pub fn from_game_state(state: &crate::api_types::GameState) -> Self {
        Self {
            snakes: state
                .board
                .snakes
                .iter()
                .map(|snake| Snake::from_battlesnake(&snake))
                .collect(),
            food: state.board.food.clone(),
            hazards: state.board.hazards.clone(),
            size: Vector::new(state.board.width, state.board.height as i32),
        }
    }
}
