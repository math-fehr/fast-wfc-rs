//! Contain the Wave struct, that keep track of main wave structure,
//! which is the possible patterns for each cell, and the entropy of the cell

use crate::utils::vec2d::Vec2D;
use crate::Real;
use std::ops::Index;

/// Contains the list of valid patterns for each cell.
/// Also, contains information about cell entropy.
pub struct Wave {
    /// The weights of patterns
    weights: Vec<Real>,
    /// The wave data. data[index][pattern] is equal to 0 if the pattern can be placed in the cell index
    data: Vec2D<Vec<bool>>,
    /// The number of possible patterns per cell
    n_patterns: Vec2D<usize>,
}

impl Wave {
    /// Create a new wave where every pattern can be in every cell.
    pub fn new(height: usize, width: usize, weights: &[Real]) -> Self {
        let n_patterns = weights.len();

        let data_cell = vec![true; n_patterns];
        let n_patterns = Vec2D::new(height, width, &n_patterns);
        let data = Vec2D::new(height, width, &data_cell);

        Wave {
            weights: weights.iter().copied().collect(),
            n_patterns,
            data,
        }
    }

    /// Return true if pattern can be placed in cell (i, j).
    pub fn get(&self, i: usize, j: usize, pattern: usize) -> bool {
        self.data[i][j][pattern]
    }

    /// Remove pattern from the wave on cell (i, j).
    /// This means that pattern cannot be placed in cell (i, j).
    pub fn unset(&mut self, i: usize, j: usize, pattern: usize) {
        if self.data[i][j][pattern] {
            self.data[i][j][pattern] = false;
            self.n_patterns[i][j] -= 1;
        }
    }

    /// Get the entropy of cell (i, j).
    pub fn get_entropy(&self, i: usize, j: usize) -> Real {
        if self.n_patterns[i][j] == 0 {
            return 0.0;
        }
        let weights: Vec<_> = self.data[i][j]
            .iter()
            .zip(self.weights.iter())
            .filter(|(b, _)| **b)
            .map(|(_, x)| *x)
            .collect();

        let sum_weight_inv: Real = 1.0 / weights.iter().sum::<Real>();

        weights.iter()
            .map(|x| x * sum_weight_inv)
            .map(|x| -x * x.ln())
            .sum()
    }

    pub fn get_min_entropy(&self) -> Option<(usize, usize)> {
        let mut min = std::f64::INFINITY as Real;
        let mut argmin = (-1, -1);

        for i in 0..self.data.height() {
            for j in 0..self.data.width() {
                let n_patterns = self.n_patterns[i][j];
                if n_patterns == 1 {
                    continue;
                }
                if n_patterns == 0 {
                    return None;
                }

                let entropy = self.get_entropy(i, j);
                if entropy < min {
                    min = entropy;
                    argmin = (i as isize, j as isize);
                }
            }
        }

        if argmin == (-1, -1) {
            None
        } else {
            Some((argmin.0 as usize, argmin.1 as usize))
        }
    }

    /// Get a reference to the actual wave data.
    pub fn data(&self) -> &Vec2D<Vec<bool>> {
        &self.data
    }
}

impl Index<usize> for Wave {
    type Output = [Vec<bool>];

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
