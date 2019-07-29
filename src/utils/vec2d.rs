//! Contain implementation of Vec2D, a 2D matrix represented by a Vec.

use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// A 2D matrix represented by a Vec.
/// The Vec contains the values line after line.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vec2D<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T> Vec2D<T> {
    /// Create a matrix given its height and width, that is filled with a value
    pub fn new(height: usize, width: usize, value: &T) -> Vec2D<T>
    where
        T: Clone,
    {
        let data = vec![value.clone(); height * width];
        Vec2D {
            height,
            width,
            data,
        }
    }

    /// Create a Vec2D given a vector representing its data, and the Vec2D size.
    /// The vector should be of the size height*width.
    pub fn from_vec(data: Vec<T>, height: usize, width: usize) -> Vec2D<T> {
        assert_eq!(height * width, data.len());
        Vec2D {
            height,
            width,
            data,
        }
    }

    /// Return an iterator on the data contained in the vec2D
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Return an iterator on the data contained in the vec2D
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Create a Vec2D using a generator function that will be called in all cells.
    pub fn from_generator<F: Fn(usize, usize) -> T>(
        height: usize,
        width: usize,
        generator: F,
    ) -> Vec2D<T> {
        let generator = &generator;
        let vec = (0..height)
            .map(|i| (0..width).map(move |j| generator(i, j)))
            .flatten()
            .collect();
        Vec2D::from_vec(vec, height, width)
    }

    /// Get the Vec2D data as a Vec.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Get the height of the Vec2D.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the width of the Vec2D.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Get the reflection along the x axis.
    pub fn reflected(&self) -> Vec2D<T>
    where
        T: Clone,
    {
        if self.width == 0 {
            return self.clone();
        }

        let data = self
            .data
            .chunks_exact(self.width)
            .map(|s| s.iter().rev())
            .flatten()
            .map(T::clone)
            .collect();
        Vec2D::from_vec(data, self.height, self.width)
    }

    /// Get the 90° anticlockwise rotation.
    pub fn rotated(&self) -> Vec2D<T>
    where
        T: Clone,
    {
        if self.data.is_empty() {
            return Vec2D::from_vec(vec![], self.width, self.height);
        }

        let mut new_vec = Vec2D::new(self.width, self.height, &self.data[0]);

        for y in 0..self.width {
            for x in 0..self.height {
                new_vec[y][x] = self[x][self.width - 1 - y].clone();
            }
        }

        new_vec
    }

    /// Get a submatrix given its upper leftmost position, and its size.
    /// The matrices are here considered toric.
    pub fn get_sub_vec(&self, y: usize, x: usize, sub_height: usize, sub_width: usize) -> Vec2D<T>
    where
        T: Clone,
    {
        if self.data.is_empty() {
            return Vec2D::from_vec(vec![], self.height, self.width);
        }

        let mut sub_vec = Vec2D::new(sub_height, sub_width, &self.data[0]);

        for dy in 0..sub_height {
            for dx in 0..sub_width {
                sub_vec[dy][dx] = self[(y + dy) % self.height][(x + dx) % self.width].clone();
            }
        }

        sub_vec
    }
}

impl<T> Index<usize> for Vec2D<T> {
    type Output = [T];

    fn index(&self, i: usize) -> &Self::Output {
        let begin_index = i * self.width;
        let end_index = (i + 1) * self.width;
        &self.data[begin_index..end_index]
    }
}

impl<T> IndexMut<usize> for Vec2D<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        let begin_index = i * self.width;
        let end_index = (i + 1) * self.width;
        &mut self.data[begin_index..end_index]
    }
}

impl<'a, T> IntoIterator for &'a Vec2D<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec2D<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mut() {
        let mut vec = Vec2D::from_vec(vec![0, 1, 2, 3, 4, 5], 2, 3);
        assert_eq!(vec.index_mut(1)[2], 5);
    }

    #[test]
    #[should_panic]
    fn test_get_mut_panic() {
        let mut vec = Vec2D::from_vec(vec![0, 1, 2, 3, 4, 5], 2, 3);
        vec.index_mut(2)[2];
    }

    #[test]
    fn test_get() {
        let vec = Vec2D::from_vec(vec![0, 1, 2, 3, 4, 5], 2, 3);
        assert_eq!(vec.index(1)[2], 5);
    }

    #[test]
    #[should_panic]
    fn test_get_panic() {
        let vec = Vec2D::from_vec(vec![0, 1, 2, 3, 4, 5], 2, 3);
        vec.index(2)[2];
    }

    #[test]
    fn test_reflected() {
        let vec = Vec2D::from_vec(vec![0, 1, 2, 3, 4, 5], 2, 3);
        let vec_result = Vec2D::from_vec(vec![2, 1, 0, 5, 4, 3], 2, 3);
        assert_eq!(vec.reflected(), vec_result);
    }

    #[test]
    fn test_reflected_empty() {
        let vec = Vec2D::<usize>::from_vec(vec![], 0, 1);
        let vec_result = Vec2D::<usize>::from_vec(vec![], 0, 1);
        assert_eq!(vec.reflected(), vec_result);
        let vec = Vec2D::<usize>::from_vec(vec![], 1, 0);
        let vec_result = Vec2D::<usize>::from_vec(vec![], 1, 0);
        assert_eq!(vec.reflected(), vec_result);
        let vec = Vec2D::<usize>::from_vec(vec![], 0, 0);
        let vec_result = Vec2D::<usize>::from_vec(vec![], 0, 0);
        assert_eq!(vec.reflected(), vec_result);
    }

    #[test]
    fn test_rotated() {
        let vec = Vec2D::from_vec(vec![0, 1, 2, 3, 4, 5], 2, 3);
        let vec_result = Vec2D::from_vec(vec![2, 5, 1, 4, 0, 3], 3, 2);
        assert_eq!(vec.rotated(), vec_result);
    }

    #[test]
    fn test_rotated_empty() {
        let vec = Vec2D::<usize>::from_vec(vec![], 0, 1);
        let vec_result = Vec2D::<usize>::from_vec(vec![], 1, 0);
        assert_eq!(vec.rotated(), vec_result);
        let vec = Vec2D::<usize>::from_vec(vec![], 1, 0);
        let vec_result = Vec2D::<usize>::from_vec(vec![], 0, 1);
        assert_eq!(vec.rotated(), vec_result);
        let vec = Vec2D::<usize>::from_vec(vec![], 0, 0);
        let vec_result = Vec2D::<usize>::from_vec(vec![], 0, 0);
        assert_eq!(vec.rotated(), vec_result);
    }
}
