use num::{self, Num};

pub trait ICoord<T: Num> {
    fn x(&self) -> T;
    fn y(&self) -> T;
    fn manhattan_distance(&self, other: &Self) -> T;
    fn mv(&self, dir: &Direction) -> Self
    where
        Self: Sized;
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
pub struct Coord<T: Num> {
    x: T,
    y: T,
}

impl<T: Num + Copy> Coord<T> {
    pub fn new(x: T, y: T) -> Self {
        Coord { x, y }
    }
}

impl<T: Num + Copy + PartialOrd> ICoord<T> for Coord<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
    }

    fn manhattan_distance(&self, other: &Self) -> T {
        let (start_x, end_x) = if self.x() < other.x() {
            (self.x(), other.x())
        } else {
            (other.x(), self.x())
        };

        let (start_y, end_y) = if self.y() < other.y() {
            (self.y(), other.y())
        } else {
            (other.y(), self.y())
        };

        end_x - start_x + end_y - start_y
    }

    fn mv(&self, dir: &Direction) -> Self {
        use Direction::*;
        match dir {
            L => Self::new(self.x() - T::one(), self.y()),
            R => Self::new(self.x() + T::one(), self.y()),
            U => Self::new(self.x(), self.y() + T::one()),
            D => Self::new(self.x(), self.y() - T::one()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    L,
    R,
    U,
    D,
}

pub struct NotAlignedWithOrientation(String);
pub enum Orientation {
    Horizontal,
    Vertical,
}
