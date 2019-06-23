//! Contain the Wave struct, that keep track of main wave structure,
//! which is the possible patterns for each cell, and the entropy of the cell

use crate::utils::vec2d::Vec2D;
use crate::Real;

/// Contains the list of valid patterns for each cell.
/// Also, contains information about cell entropy.
pub struct Wave {
    /// The precomputation of frequency * log(frequency)
    plogp_weights: Vec<Real>,
    /// The wave data. data[index][pattern] is equal to 0 if the pattern can be placed in the cell index
    data: Vec2D<Vec<bool>>,
}

impl Wave {
    /// Create a new wave where every pattern can be in every cell.
    pub fn new(height: usize, width: usize, weights: Vec<Real>) -> Self {
        let n_patterns = weights.len();
        let plogp_weights = weights.clone().into_iter().map(|s| s * s.ln()).collect();

        let data_cell = vec![true; n_patterns];
        let data = Vec2D::new(height, width, &data_cell);

        Wave {
            plogp_weights,
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
        self.data[i][j][pattern] = false;
    }

    /// Get the entropy of cell (i, j).
    pub fn get_entropy(&self, i: usize, j: usize) -> Real {
        self.data[i][j]
            .iter()
            .zip(self.plogp_weights.iter())
            .map(|(b, plogp)| if *b { *plogp } else { 0 as Real })
            .sum()
    }
}
