//! A direction parallel to the axes in a 2D space

use Direction::{Down, Left, Right, Up};

/// The enum representing a direction parallel to the axes in a 2D space
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Down = 0,
    Left = 1,
    Right = 2,
    Up = 3,
}

impl Direction {
    /// Get the opposite direction
    pub fn opposite(self) -> Self {
        unsafe { std::mem::transmute(3 - (self as u8)) }
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
}
