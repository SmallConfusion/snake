use rocket::serde::Serialize;

use super::Vector;

#[derive(Clone, Copy, Debug, strum::Display, PartialEq, Eq, strum::EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn get_str(&self) -> &'static str {
        match self {
            Direction::Up => "up",
            Direction::Right => "right",
            Direction::Down => "down",
            Direction::Left => "left",
        }
    }

    pub fn get_vector(&self) -> Vector {
        match self {
            Direction::Up => Vector::new(0, -1),
            Direction::Right => Vector::new(1, 0),
            Direction::Down => Vector::new(0, 1),
            Direction::Left => Vector::new(-1, 0),
        }
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.get_str())
    }
}
