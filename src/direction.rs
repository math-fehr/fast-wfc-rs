//! A direction parallel to the axes in a 2D space

use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use Direction::{Down, Left, Right, Up};

/// The enum representing a direction parallel to the axes in a 2D space
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Down = 0,
    Left = 1,
    Right = 2,
    Up = 3,
}

impl Direction {
    /// Get the opposite direction
    pub fn opposite(self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
        }
    }

    /// Get all possible directions
    pub fn directions() -> [Direction; 4] {
        [Down, Left, Right, Up]
    }

    /// Get the (y,x) coordinates of the normalized vector representing the direction.
    pub fn get_coordinates(self) -> (isize, isize) {
        match self {
            Down => (-1, 0),
            Left => (0, -1),
            Right => (0, 1),
            Up => (1, 0),
        }
    }
}

/// An array that is indexed by a direction
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct DirArray<T> {
    data: [T; 4],
}

impl<T> DirArray<T> {
    /// Create a new array given a default value that will be assigned to
    /// each direction.
    pub fn new(default: &T) -> DirArray<T>
    where
        T: Clone,
    {
        DirArray {
            data: [
                default.clone(),
                default.clone(),
                default.clone(),
                default.clone(),
            ],
        }
    }

    /// Create a new array where the values assigned to each direction
    /// is given by the given generator
    pub fn new_generator<F: Fn(Direction) -> T>(generator: F) -> DirArray<T> {
        DirArray {
            data: [
                generator(Down),
                generator(Left),
                generator(Right),
                generator(Up),
            ],
        }
    }

    /// Modify the data according to the given closure.
    pub fn map<U, F: Fn(T) -> U>(self, f: F) -> DirArray<U> {
        let [v0, v1, v2, v3] = self.data;
        DirArray {
            data: [f(v0), f(v1), f(v2), f(v3)],
        }
    }
}

impl<T> Index<Direction> for DirArray<T> {
    type Output = T;

    fn index(&self, dir: Direction) -> &Self::Output {
        &self.data[dir as u8 as usize]
    }
}

impl<'a, T> IntoIterator for &'a DirArray<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DirArray<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> IndexMut<Direction> for DirArray<T> {
    fn index_mut(&mut self, dir: Direction) -> &mut Self::Output {
        &mut self.data[dir as u8 as usize]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_opposite() {
        assert_eq!(Down.opposite(), Up);
        assert_eq!(Up.opposite(), Down);
        assert_eq!(Left.opposite(), Right);
        assert_eq!(Right.opposite(), Left);
    }

    #[test]
    fn test_index() {
        let mut array = DirArray::new(&3);
        assert_eq!(array[Down], 3);
        array[Up] = 2;
        assert_eq!(array[Up], 2);
    }

    #[test]
    fn test_generator() {
        let array = DirArray::new_generator(|dir| match dir {
            Up => 0,
            Left => 1,
            Down => 2,
            Right => 3,
        });
        assert_eq!(array[Up], 0);
        assert_eq!(array[Left], 1);
        assert_eq!(array[Down], 2);
        assert_eq!(array[Right], 3);
    }
}
