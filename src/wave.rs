//! Contain the Wave struct, that keep track of main wave structure,
//! which is the possible patterns for each cell, and the entropy of the cell

use crate::Real;
use crate::utils::vec2d::Vec2D;

/// Struct keeping track of the values needed to compute the entropy of each
/// cell efficiently.
/// Here, p(pattern) is 0 if it is not valid for the cell, and equal to the
/// frequency of the pattern otherwise.
struct EntropyMemoisation {
    /// The sum of p(pattern) * log(p(pattern))
    plogp_sum: Vec<Real>,
    /// The sum of p(pattern)
    sum: Vec<Real>,
    // The log of sum
    log_sum: Vec<Real>,
    // The number of valid patterns
    nb_patterns: Vec<usize>,
    // The entropy of the cell
    entropy: Vec<Real>,
}

/// Contains the list of valid patterns for each cell.
/// Also, contains information about cell entropy.
struct Wave {
    /// The pattern frequencies.
    frequencies: Vec<Real>,
    /// The precomputation of frequency * log(frequency)
    plogp_frequencies: Vec<Real>,
    /// The precomputation of min (frequency * log(frequency)) / 2
    min_abs_half_plogp: Real,
    /// The memoisation of important values for the entropy computation
    memoisation: EntropyMemoisation,
    /// The number of distinct patterns
    n_patterns: usize,
    /// The wave data. data[index][pattern] is equal to 0 if the pattern can be placed in the cell index
    data: Vec2D<u8>,
}
