use rocket::serde::Serialize;
use serde::ser::SerializeStruct;

use super::Vector;

#[derive(Clone, Copy, Debug, strum::Display, PartialEq, Eq, strum::EnumIter)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn from_vector(vector: &Vector) -> Option<Self> {
        match vector {
            Vector { x: 0, y: 1 } => Some(Self::Up),
            Vector { x: 1, y: 0 } => Some(Self::Right),
            Vector { x: 0, y: -1 } => Some(Self::Down),
            Vector { x: -1, y: 0 } => Some(Self::Left),
            _ => None,
        }
    }

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
            Direction::Up => Vector::new(0, 1),
            Direction::Right => Vector::new(1, 0),
            Direction::Down => Vector::new(0, -1),
            Direction::Left => Vector::new(-1, 0),
        }
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Move", 1).unwrap();
        s.serialize_field("move", self.get_str()).unwrap();
        s.end()
    }
}
