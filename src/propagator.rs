//! Contain the Propagator stuct, that propagates information through the wave.

use crate::wave::Wave;
use crate::Real;
use crate::direction::*;

/// Propagator is a wrapper around Wave, that ensure that the constraints between
/// neighbors are respected.
pub struct Propagator {
    /// The wave we propagate information in.
    wave: Wave,
    /// Is the wave toric.
    is_toric: bool,
    /// patterns_compatibility[pattern1][dir][pattern2] is true
    /// if pattern1 can be placed in direction dir of pattern2.
    patterns_compatibility: Vec<[Vec<bool>; 4]>,
    /// compatible[y][x][pattern][dir] contains the number of distincts patterns
    /// in the wave that can be placed in the cell at direction dir of (y,x), without
    /// being in contradiction with pattern placed in (y,x). If wave[y][x][pattern]
    /// is false, then compatible[y][x][pattern] has every element negative or null.
    compatible: Vec<Vec<Vec<[usize; 4]>>>,
    /// The set of tuples (y, x, pattern) that should be propagated.
    /// Such a tuple should be propagated if wave[y][x][pattern] is set to false.
    propagating_queue: Vec<(usize, usize, usize)>,
}

impl Propagator {
    pub fn new(
        height: usize,
        width: usize,
        weights: Vec<Real>,
        patterns_compatibility: Vec<[Vec<bool>; 4]>,
        is_toric: bool,
    ) -> Propagator {
        let n_patterns = weights.len();
        let wave = Wave::new(height, width, weights);

        let compatible = (0..height).map(|_| {
            (0..width).map(|_| {
                (0..n_patterns).map(|pattern| {
                    let mut array = [0; 4];
                    for direction in &Direction::directions() {
                        array[*direction as u8 as usize] = patterns_compatibility[pattern][direction.opposite() as u8 as usize].len()
                    }
                    array
                }).collect()
            }).collect()
        }).collect();

        Propagator {
            wave,
            is_toric,
            patterns_compatibility,
            compatible,
            propagating_queue: vec![],
        }
    }
}
