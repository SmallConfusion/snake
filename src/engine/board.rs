use log::{error, warn};

use super::{Direction, Vector};
use std::collections::VecDeque;

#[derive(Clone, PartialEq, Debug)]
pub struct Snake {
    pub body: VecDeque<Vector>,
    pub health: i32,
}

impl Snake {
    pub fn from_battlesnake(snake: &crate::api_types::Battlesnake) -> Self {
        Snake {
            body: snake.body.clone().into(),
            health: snake.health,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Board {
    pub snakes: Vec<Snake>, // First one is us
    pub food: Vec<Vector>,
    pub hazards: Vec<Vector>,
    pub size: Vector,
    pub player_dead: bool,
}

impl Board {
    pub fn from_game_state(state: &crate::api_types::GameState) -> Self {
        let mut snakes: Vec<Snake> = vec![Snake::from_battlesnake(&state.you)];

        for battlesnake in state.board.snakes.iter().filter(|s| s.id != state.you.id) {
            snakes.push(Snake::from_battlesnake(battlesnake));
        }

        Self {
            snakes,
            food: state.board.food.clone(),
            hazards: state.board.hazards.clone(),
            size: Vector::new(state.board.width, state.board.height as i32),
            player_dead: false,
        }
    }

    pub fn iterate(&mut self, moves: &[Direction]) {
        if moves.len() != self.snakes.len() {
            warn!("There aren't the same number of moves as snakes");
        }

        for (i, snake) in self.snakes.iter_mut().enumerate() {
            // Move forward
            let dir = moves.get(i).unwrap_or(&Direction::Up);
            let dir_v = dir.get_vector();

            let head = match snake.body.get(0) {
                Some(h) => *h,
                None => {
                    error!("Snake has no body, iteration failed.");
                    return;
                }
            };

            let next_pos = head + dir_v;

            snake.body.pop_back();
            snake.body.push_front(next_pos);

            snake.health -= 1;

            // Food
            if self.food.contains(&snake.body[0]) {
                snake.body.push_back(*snake.body.back().unwrap());
                snake.health = 100;
            }
        }

        // Remove foods
        self.food
            .retain(|food| !self.snakes.iter().any(|s| &s.body[0] == food));

        // Kill
        let keep: Vec<bool> = self
            .snakes
            .iter()
            .map(|s| {
                // Borders
                if s.body[0].x < 0
                    || s.body[0].y < 0
                    || s.body[0].x >= self.size.x
                    || s.body[0].y >= self.size.y
                {
                    return false;
                }

                // Head body
                if self
                    .snakes
                    .iter()
                    .any(|sj| sj.body.iter().skip(1).any(|&p| p == s.body[0]))
                {
                    return false;
                }

                // Head head
                if self.snakes.iter().filter(|sj| s != *sj).any(|sj| {
                    if sj.body[0] == s.body[0] {
                        sj.body.len() >= s.body.len()
                    } else {
                        false
                    }
                }) {
                    return false;
                }

                true
            })
            .collect();

        if !keep[0] {
            self.player_dead = true;
        }

        // trace!("{keep:?}");

        let mut keep_iter = keep.iter();

        self.snakes.retain(|_| *keep_iter.next().unwrap());
    }
}
