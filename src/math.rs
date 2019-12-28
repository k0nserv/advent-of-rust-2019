use std::fmt;
use std::hash::Hash;
use std::ops::{Add, Neg, Sub};

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Default> Default for Vector2<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<Vector2 x={:?} y={:?} >", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::<T> {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::<T> {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Neg<Output = T> + Hash + PartialEq> Neg for Vector2<T> {
    type Output = Vector2<T>;

    fn neg(self) -> Self::Output {
        Vector2::<T> {
            x: -self.x,
            y: -self.y,
        }
    }
}

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

macro_rules! define_abs {
    ($T:ident) => {
        impl Abs for $T {
            type Output = $T;

            fn abs(self) -> Self::Output {
                self.abs()
            }
        }
    };
}

define_abs!(isize);

impl<T: Abs<Output = T> + Sub<Output = T> + Add<Output = T>> Vector2<T> {
    pub fn manhattan_distance(self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
