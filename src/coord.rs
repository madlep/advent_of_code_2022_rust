use num;
use num::Num;

pub trait ICoord<T: Num> {
    fn x(&self) -> T;
    fn y(&self) -> T;
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

impl<T: Num + Copy> ICoord<T> for Coord<T> {
    fn x(&self) -> T {
        self.x
    }

    fn y(&self) -> T {
        self.y
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
