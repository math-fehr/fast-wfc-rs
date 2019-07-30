//! Contain the Wave struct, that keep track of main wave structure,
//! which is the possible patterns for each cell, and the entropy of the cell

use crate::utils::vec2d::Vec2D;
use crate::Real;
use std::ops::Index;

/// Values memoized to compute the entropy. Keeping these allow us to update quickly
/// the entropy when modifying the wave.
#[derive(Clone, Copy)]
struct EntropyMemoizationCell {
    /// The sum of p(pattern) * (log(p(pattern)))
    plogp_sum: Real,
    /// The sum of p(pattern)
    sum: Real,
    /// The number of possible patterns
    n_patterns: usize,
}

impl EntropyMemoizationCell {
    /// Update the values when removing a pattern of weight weight.
    fn update(&mut self, weight: Real) {
        self.plogp_sum -= weight * weight.ln();
        self.sum -= weight;
        self.n_patterns -= 1;
    }

    /// Get the entropy
    fn entropy(&self) -> Real {
        self.sum.ln() - (self.plogp_sum / self.sum)
    }
}

/// Values memoized to compute the entropy for each cell.
struct EntropyMemoization {
    /// The memoization for each cell
    data: Vec2D<EntropyMemoizationCell>,
}

impl EntropyMemoization {
    /// Create a new object given the weights of the patterns used in the wave.
    fn new(weights: &[Real], height: usize, width: usize) -> EntropyMemoization {
        let sum = weights.iter().sum();
        let plogp_sum = weights.iter().map(|x| x * x.ln()).sum();
        let n_patterns = weights.len();
        let memoization_cell = EntropyMemoizationCell {
            plogp_sum,
            sum,
            n_patterns,
        };
        EntropyMemoization {
            data: Vec2D::new(height, width, &memoization_cell),
        }
    }

    /// Update the memoized values for a cell.
    fn update(&mut self, y: usize, x: usize, weight: Real) {
        self.data[y][x].update(weight)
    }

    /// Get the entropy of a cell.
    fn entropy(&self, y: usize, x: usize) -> Real {
        self.data[y][x].entropy()
    }
}

/// Contains the list of valid patterns for each cell.
/// Also, contains information about cell entropy.
pub struct Wave {
    /// The wave data. data[index][pattern] is equal to 0 if the pattern can be placed in the cell index
    data: Vec2D<Vec<bool>>,
    /// The weigths of each pattern
    weights: Vec<Real>,
    /// The values memoized to compute the entropy of each cell
    entropy_memoization: EntropyMemoization,
}

/// Error for some operations dealing with the wave.
/// Impossible mean that there is a contradiction in the wave, and no solution exists.
/// Finished mean that every cell is determined
pub enum WaveError {
    Impossible,
    Finished,
}

impl Wave {
    /// Create a new wave where every pattern can be in every cell.
    pub fn new(height: usize, width: usize, weights: Vec<Real>) -> Self {
        let entropy_memoization = EntropyMemoization::new(&weights, height, width);
        Wave {
            data: Vec2D::new(height, width, &vec![true; weights.len()]),
            weights,
            entropy_memoization: entropy_memoization,
        }
    }

    /// Set every element in the wave to true
    pub fn reset(&mut self) {
        for v in &mut self.data {
            for i in v {
                *i = true;
            }
        }
        self.entropy_memoization =
            EntropyMemoization::new(&self.weights, self.data.height(), self.data.width());
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
            self.entropy_memoization.update(i, j, self.weights[pattern]);
        }
    }

    /// Get the entropy of cell (i, j).
    pub fn get_entropy(&self, i: usize, j: usize) -> Real {
        self.entropy_memoization.entropy(i, j)
    }

    pub fn get_min_entropy(&self) -> Result<(usize, usize), WaveError> {
        let mut min = std::f64::INFINITY as Real;
        let mut argmin = (-1, -1);

        for i in 0..self.data.height() {
            for j in 0..self.data.width() {
                let n_patterns = self.entropy_memoization.data[i][j].n_patterns;
                if n_patterns == 1 {
                    continue;
                }
                if n_patterns == 0 {
                    return Err(WaveError::Impossible);
                }

                let entropy = self.get_entropy(i, j);
                if entropy < min {
                    min = entropy;
                    argmin = (i as isize, j as isize);
                }
            }
        }

        if argmin == (-1, -1) {
            Err(WaveError::Finished)
        } else {
            Ok((argmin.0 as usize, argmin.1 as usize))
        }
    }

    /// Get the wave height
    pub fn height(&self) -> usize {
        self.data.height()
    }

    /// Get the wave width
    pub fn width(&self) -> usize {
        self.data.width()
    }
}

impl Index<usize> for Wave {
    type Output = [Vec<bool>];

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
