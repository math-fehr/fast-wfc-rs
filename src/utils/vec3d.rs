//! Contains the Vec3D implementation, a 3D matrix represented by a Vec.

use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};

/// A 3D matrix represented by a Vec.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Vec3D<T> {
    depth: usize,
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T> Vec3D<T> {
    /// Create a matrix given its height and width, that is filled with a value
    pub fn new(depth: usize, height: usize, width: usize, value: &T) -> Vec3D<T>
    where
        T: Clone,
    {
        let data = vec![value.clone(); depth * height * width];
        Vec3D {
            depth,
            height,
            width,
            data,
        }
    }

    /// Create a Vec3D given a vector representing its data, and the Vec3D size.
    /// The vector should be of the size height*width*depth.
    pub fn from_vec(data: Vec<T>, depth: usize, height: usize, width: usize) -> Vec3D<T> {
        assert_eq!(depth * height * width, data.len());
        Vec3D {
            depth,
            height,
            width,
            data,
        }
    }

    /// Create a Vec2D using a generator function that will be called in all cells.
    pub fn new_generator<F: Fn(usize, usize, usize) -> T>(
        depth: usize,
        height: usize,
        width: usize,
        generator: F,
    ) -> Vec3D<T> {
        let generator = &generator;
        let vec = (0..depth)
            .map(|i| (0..height).map(move |j| (0..width).map(move |k| generator(i, j, k))))
            .flatten()
            .flatten()
            .collect();
        Vec3D::from_vec(vec, depth, height, width)
    }

    /// Return an iterator on the data contained in the vec3D
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Return an iterator on the data contained in the vec3D
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Get the Vec3D data as a Vec.
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }

    /// Get the size of the first dimension.
    pub fn depth(&self) -> usize {
        self.depth
    }

    /// Get the size of the second dimension.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Get the size of the third dimension.
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, i: usize, j: usize, k: usize) -> &T {
        &self.data[k + self.width * (j + i * self.height)]
    }

    pub fn get_mut(&mut self, i: usize, j: usize, k: usize) -> &mut T {
        &mut self.data[k + self.width * (j + i * self.height)]
    }
}

impl<'a, T> IntoIterator for &'a Vec3D<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Vec3D<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

impl<T> Index<(usize, usize)> for Vec3D<T> {
    type Output = [T];

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        let begin_index = self.width * (j + i * self.height);
        let end_index = self.width * (1 + j + i * self.height);
        &self.data[begin_index..end_index]
    }
}

impl<T> IndexMut<(usize, usize)> for Vec3D<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        let begin_index = self.width * (j + i * self.height);
        let end_index = self.width * (1 + j + i * self.height);
        &mut self.data[begin_index..end_index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let vec = Vec3D::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 2, 3, 2);
        assert_eq!(*vec.get(0, 2, 1), 5);
        assert_eq!(*vec.get(1, 0, 1), 7);
    }

    #[test]
    fn test_index() {
        let vec = Vec3D::from_vec(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11], 2, 3, 2);
        assert_eq!(vec[(0, 2)][1], 5);
        assert_eq!(vec[(1, 0)][1], 7);
    }
}
