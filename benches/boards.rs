use snake::engine::{Board, Snake, Vector};
use std::collections::VecDeque;

pub fn get_board_2() -> Board {
    Board {
        food: vec![Vector::new(7, 6)],
        snakes: vec![
            Snake {
                body: VecDeque::from([
                    Vector::new(5, 5),
                    Vector::new(4, 5),
                    Vector::new(3, 5),
                    Vector::new(3, 4),
                    Vector::new(2, 4),
                    Vector::new(2, 5),
                    Vector::new(2, 6),
                    Vector::new(3, 6),
                    Vector::new(3, 7),
                    Vector::new(3, 8),
                ]),
                health: 86,
            },
            Snake {
                body: VecDeque::from([
                    Vector::new(5, 1),
                    Vector::new(6, 1),
                    Vector::new(6, 0),
                    Vector::new(5, 0),
                    Vector::new(4, 0),
                    Vector::new(3, 0),
                    Vector::new(2, 0),
                    Vector::new(1, 0),
                    Vector::new(1, 1),
                ]),
                health: 93,
            },
        ],
        hazards: vec![],
        size: Vector::new(11, 11),
        player_dead: false,
    }
}

pub fn get_board_4() -> Board {
    // https://play.battlesnake.com/game/cda6cb45-0836-41fa-b1e9-292478478da3?turn=108
    Board {
        food: vec![Vector::new(7, 6)],
        snakes: vec![
            Snake {
                body: VecDeque::from([
                    Vector::new(5, 5),
                    Vector::new(4, 5),
                    Vector::new(3, 5),
                    Vector::new(3, 4),
                    Vector::new(2, 4),
                    Vector::new(2, 5),
                    Vector::new(2, 6),
                    Vector::new(3, 6),
                    Vector::new(3, 7),
                    Vector::new(3, 8),
                ]),
                health: 86,
            },
            Snake {
                body: VecDeque::from([
                    Vector::new(5, 1),
                    Vector::new(6, 1),
                    Vector::new(6, 0),
                    Vector::new(5, 0),
                    Vector::new(4, 0),
                    Vector::new(3, 0),
                    Vector::new(2, 0),
                    Vector::new(1, 0),
                    Vector::new(1, 1),
                ]),
                health: 93,
            },
            Snake {
                body: VecDeque::from([
                    Vector::new(10, 10),
                    Vector::new(9, 10),
                    Vector::new(9, 9),
                    Vector::new(8, 9),
                    Vector::new(8, 8),
                    Vector::new(8, 7),
                    Vector::new(8, 6),
                    Vector::new(8, 5),
                    Vector::new(8, 4),
                    Vector::new(7, 4),
                ]),
                health: 99,
            },
            Snake {
                body: VecDeque::from([
                    Vector::new(5, 9),
                    Vector::new(5, 10),
                    Vector::new(4, 10),
                    Vector::new(3, 10),
                    Vector::new(2, 10),
                    Vector::new(1, 10),
                    Vector::new(0, 10),
                    Vector::new(0, 9),
                    Vector::new(1, 9),
                ]),
                health: 95,
            },
        ],
        hazards: vec![],
        size: Vector::new(11, 11),
        player_dead: false,
    }
}
