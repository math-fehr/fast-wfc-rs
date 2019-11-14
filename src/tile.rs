//! Contains the Symmetry implementation, and the Tile implementation.

use crate::utils::vec2d::*;
use crate::Real;

/// The different kind of symmetries a 2D object can have.
#[derive(Clone, Copy)]
pub enum Symmetry {
    X,
    I,
    Backslash,
    T,
    L,
    P,
}

impl Symmetry {
    /// Get the possible number of orientations a kind of symmetry has.
    /// An orientation is a combination of rotation and reflection that
    /// lead to a different object.
    pub fn nb_of_possible_orientations(self) -> usize {
        match self {
            Symmetry::X => 1,
            Symmetry::I | Symmetry::Backslash => 2,
            Symmetry::T | Symmetry::L => 4,
            Symmetry::P => 8,
        }
    }
}

/// Generate the map associating an orientation to the orientation
/// obtained when rotating 90° anticlockwise the orientation.
fn generate_rotation_map(symmetry: Symmetry) -> Vec<usize> {
    match symmetry {
        Symmetry::X => vec![0],
        Symmetry::I | Symmetry::Backslash => vec![1, 0],
        Symmetry::T | Symmetry::L => vec![1, 2, 3, 0],
        Symmetry::P => vec![1, 2, 3, 0, 5, 6, 7, 4],
    }
}

/// Generate the map associating an orientation to the orientation obtained
/// by reflecting the object along the x axis.
fn generate_reflection_map(symmetry: Symmetry) -> Vec<usize> {
    match symmetry {
        Symmetry::X => vec![0],
        Symmetry::I => vec![0, 1],
        Symmetry::Backslash => vec![1, 0],
        Symmetry::T => vec![0, 3, 2, 1],
        Symmetry::L => vec![1, 0, 3, 2],
        Symmetry::P => vec![4, 7, 6, 5, 0, 3, 2, 1],
    }
}

/// Generate the map associating an orientation and an action to the resulting orientation.
/// An action is a sequence of rotations and reflections.
/// Actions 0 to 3 are 0°, 90°, 180°, and 270° anticlockwise rotations.
/// Actions 4 to 7 are actions 0 to 3 preceded by a reflection on the x axis.
pub fn generate_action_map(symmetry: Symmetry) -> Vec<Vec<usize>> {
    let rotation_map = generate_rotation_map(symmetry);
    let reflection_map = generate_reflection_map(symmetry);
    let size = rotation_map.len();
    let mut action_map = vec![vec![0; size]; 8];

    for i in 0..size {
        action_map[0][i] = i;
    }

    for a in 1..4 {
        for i in 0..size {
            action_map[a][i] = rotation_map[action_map[a - 1][i]];
        }
    }

    for i in 0..size {
        action_map[4][i] = reflection_map[action_map[0][i]];
    }

    for a in 5..8 {
        for i in 0..size {
            action_map[a][i] = rotation_map[action_map[a - 1][i]];
        }
    }

    action_map
}

/// Generate all distincts orientations of a 2D array given its symmetry type.
pub fn generate_oriented<T>(data: Vec2D<T>, symmetry: Symmetry) -> Vec<Vec2D<T>>
where
    T: Clone,
{
    match symmetry {
        Symmetry::X => vec![data],
        Symmetry::I | Symmetry::Backslash => {
            let rotated = data.rotated();
            vec![data, rotated]
        }
        Symmetry::T | Symmetry::L => {
            let mut oriented = vec![data];
            for _ in 0..3 {
                oriented.push(oriented.last().unwrap().rotated())
            }
            oriented
        }
        Symmetry::P => {
            let mut oriented = vec![data];
            for _ in 0..3 {
                oriented.push(oriented.last().unwrap().rotated())
            }

            oriented.push(oriented.last().unwrap().reflected());
            for _ in 0..3 {
                oriented.push(oriented.last().unwrap().rotated())
            }
            oriented
        }
    }
}

/// 2D Objects that are reflections and rotations of themselves.
/// Item i is obtained by doing action i on item 0.
/// See [generate_action_map] to see what actions do.
#[derive(Clone)]
pub struct Tile<T> {
    data: Vec<Vec2D<T>>,
    symmetry: Symmetry,
    weight: Real,
}

impl<T> Tile<T> {
    /// Create a new tile given a Vec2D representing an object.
    pub fn new(data: Vec2D<T>, symmetry: Symmetry, weight: Real) -> Tile<T>
    where
        T: Clone,
    {
        let oriented_data = generate_oriented(data, symmetry);
        Tile {
            data: oriented_data,
            symmetry,
            weight,
        }
    }

    /// Get the different rotations of the object represented by the tile.
    pub fn data(&self) -> &Vec<Vec2D<T>> {
        &self.data
    }

    /// Get the symmetry kinds of the tile.
    pub fn symmetry(&self) -> Symmetry {
        self.symmetry
    }

    /// Get the weight of the tile
    pub fn weight(&self) -> Real {
        self.weight
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_oriented() {
        let data = Vec2D::from_vec(vec![1, 2], 1, 2);
        let oriented_data = generate_oriented(data, Symmetry::T);

        assert_eq!(oriented_data[0], Vec2D::from_vec(vec![1, 2], 1, 2));
        assert_eq!(oriented_data[1], Vec2D::from_vec(vec![2, 1], 2, 1));
        assert_eq!(oriented_data[2], Vec2D::from_vec(vec![2, 1], 1, 2));
        assert_eq!(oriented_data[3], Vec2D::from_vec(vec![1, 2], 2, 1));
    }
}
