#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Vec2D<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T> Vec2D<T> {
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

    pub fn from_vec(height: usize, width: usize, data: Vec<T>) -> Vec2D<T> {
        assert!(height * width == data.len());
        Vec2D {
            height,
            width,
            data,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn get(&self, y: usize, x: usize) -> &T {
        assert!(y < self.height && x < self.width);
        &self.data[x + y * self.width]
    }

    pub fn get_mut(&mut self, y: usize, x: usize) -> &mut T {
        assert!(y < self.height && x < self.width);
        &mut self.data[x + y * self.width]
    }

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
        Vec2D::from_vec(self.height, self.width, data)
    }

    pub fn rotated(&self) -> Vec2D<T>
    where
        T: Clone,
    {
        if self.data.len() == 0 {
            return Vec2D::from_vec(self.width, self.height, vec![]);
        }

        let mut new_vec = Vec2D::new(self.width, self.height, &self.data[0]);

        for y in 0..self.width {
            for x in 0..self.height {
                *new_vec.get_mut(y, x) = self.get(x, self.width - 1 - y).clone();
            }
        }

        new_vec
    }

    pub fn get_sub_vec(&self, y: usize, x: usize, sub_height: usize, sub_width: usize) -> Vec2D<T>
    where
        T: Clone,
    {
        assert!(sub_width <= self.width);
        assert!(sub_height <= self.height);

        if self.data.len() == 0 {
            return Vec2D::from_vec(self.height, self.width, vec![]);
        }

        let mut sub_vec = Vec2D::new(sub_height, sub_width, &self.data[0]);

        for dy in 0..sub_height {
            for dx in 0..sub_width {
                *sub_vec.get_mut(dy, dx) = self
                    .get((y + dy) % self.height, (x + dx) % self.width)
                    .clone();
            }
        }

        sub_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mut() {
        let mut vec = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
        assert!(*vec.get_mut(1, 2) == 5);
    }

    #[test]
    #[should_panic]
    fn test_get_mut_panic() {
        let mut vec = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
        vec.get_mut(2, 2);
    }

    #[test]
    fn test_get() {
        let vec = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
        assert!(*vec.get(1, 2) == 5);
    }

    #[test]
    #[should_panic]
    fn test_get_panic() {
        let vec = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
        vec.get(2, 2);
    }

    #[test]
    fn test_reflected() {
        let vec = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
        let vec_result = Vec2D::from_vec(2, 3, vec![2, 1, 0, 5, 4, 3]);
        assert!(vec.reflected() == vec_result);
    }

    #[test]
    fn test_reflected_empty() {
        let vec = Vec2D::<usize>::from_vec(0, 1, vec![]);
        let vec_result = Vec2D::<usize>::from_vec(0, 1, vec![]);
        assert!(vec.reflected() == vec_result);
        let vec = Vec2D::<usize>::from_vec(1, 0, vec![]);
        let vec_result = Vec2D::<usize>::from_vec(1, 0, vec![]);
        assert!(vec.reflected() == vec_result);
        let vec = Vec2D::<usize>::from_vec(0, 0, vec![]);
        let vec_result = Vec2D::<usize>::from_vec(0, 0, vec![]);
        assert!(vec.reflected() == vec_result);
    }

    #[test]
    fn test_rotated() {
        let vec = Vec2D::from_vec(2, 3, vec![0, 1, 2, 3, 4, 5]);
        let vec_result = Vec2D::from_vec(3, 2, vec![2, 5, 1, 4, 0, 3]);
        assert!(vec.rotated() == vec_result);
    }

    #[test]
    fn test_rotated_empty() {
        let vec = Vec2D::<usize>::from_vec(0, 1, vec![]);
        let vec_result = Vec2D::<usize>::from_vec(1, 0, vec![]);
        assert!(vec.rotated() == vec_result);
        let vec = Vec2D::<usize>::from_vec(1, 0, vec![]);
        let vec_result = Vec2D::<usize>::from_vec(0, 1, vec![]);
        assert!(vec.rotated() == vec_result);
        let vec = Vec2D::<usize>::from_vec(0, 0, vec![]);
        let vec_result = Vec2D::<usize>::from_vec(0, 0, vec![]);
        assert!(vec.rotated() == vec_result);
    }
}
