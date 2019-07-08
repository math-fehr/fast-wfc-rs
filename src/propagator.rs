//! Contain the Propagator stuct, that propagates information through the wave.

use crate::direction::*;
use crate::wave::Wave;
use crate::Real;

/// Propagator is a wrapper around Wave, that ensure that the constraints between
/// neighbors are respected.
pub struct Propagator {
    /// The wave we propagate information in.
    wave: Wave,
    /// Is the wave toric.
    is_toric: bool,
    /// patterns_compatibility[pattern1][dir][pattern2] is true
    /// if pattern1 can be placed in direction dir of pattern2.
    patterns_compatibility: Vec<[Vec<usize>; 4]>,
    /// compatible[y][x][pattern][dir] contains the number of distincts patterns
    /// in the wave that can be placed in the cell at direction dir of (y,x), without
    /// being in contradiction with pattern placed in (y,x). If wave[y][x][pattern]
    /// is false, then compatible[y][x][pattern] has every element negative or null.
    compatible: Vec<Vec<Vec<[isize; 4]>>>,
    /// The set of tuples (y, x, pattern) that should be propagated.
    /// Such a tuple should be propagated if wave[y][x][pattern] is set to false.
    propagating_queue: Vec<(usize, usize, usize)>,
}

impl Propagator {
    /// Create a new Propagator, given the weights of the patterns,
    /// and the possible combinations of pair of patterns.
    pub fn new(
        height: usize,
        width: usize,
        weights: &[Real],
        patterns_compatibility: Vec<[Vec<usize>; 4]>,
        is_toric: bool,
    ) -> Propagator {
        let n_patterns = weights.len();
        let wave = Wave::new(height, width, weights);

        let compatible = (0..height)
            .map(|_| {
                (0..width)
                    .map(|_| {
                        (0..n_patterns)
                            .map(|pattern| {
                                let mut array = [0; 4];
                                for direction in &Direction::directions() {
                                    array[*direction as u8 as usize] = patterns_compatibility
                                        [pattern]
                                        [direction.opposite() as u8 as usize]
                                        .len() as isize
                                }
                                array
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();

        Propagator {
            wave,
            is_toric,
            patterns_compatibility,
            compatible,
            propagating_queue: vec![],
        }
    }

    /// Remove pattern from the wave on cell (i, j).
    /// This means that pattern cannot be placed in cell (i, j).
    pub fn unset(&mut self, y: usize, x: usize, pattern: usize) {
        self.wave.unset(y, x, pattern);
        self.compatible[y][x][pattern] = [0; 4];
        self.propagating_queue.push((y, x, pattern));
        self.propagate();
    }

    /// Propagate the information collected by the unset functions.
    fn propagate(&mut self) {
        // We propagate as long as we have things to propagate.
        // (y1, x1) is the cell where pattern was set to false in the wave.
        while let Some((y1, x1, pattern)) = self.propagating_queue.pop() {
            for direction in &Direction::directions() {

                let (dy, dx) = direction.get_coordinates();
                let direction = *direction as u8 as usize;

                // The coordinate of a neighboring cell
                let (y2, x2) = if self.is_toric {
                    (
                        (y1 as isize + dy + self.wave.data().height() as isize) as usize,
                        (x1 as isize + dx + self.wave.data().width() as isize) as usize,
                    )
                } else {
                    let (y2, x2) = (y1 as isize + dy, x1 as isize + dx);
                    if x2 < 0 || x2 >= self.wave.data().width() as isize {
                        continue;
                    }
                    if y2 < 0 || y2 >= self.wave.data().height() as isize {
                        continue;
                    }
                    (y2 as usize, x2 as usize)
                };

                // We iterate on every pattern that could be placed in the (y2, x2) cell,
                // without being in contradiction with pattern in (y1, x1)
                for &pattern2 in &self.patterns_compatibility[pattern][direction] {

                    // We decrease the number of compatible patterns in the opposite
                    // direction. If the pattern was discarded from the wave, the element is
                    // negative.
                    let value = &mut self.compatible[y2][x2][pattern2];
                    value[direction] -= 1;

                    // If the elemnt was set to 0 with this operation, we need to remove the
                    // pattern from the wave, and propagate the newly acquired information.
                    if value[direction] == 0 {
                        // We can't call self.unset here, because self is already borrowed.
                        self.wave.unset(y2, x2, pattern2);
                        *value = [0; 4];
                        self.propagating_queue.push((y2, x2, pattern2));
                    }
                }
            }
        }
    }
}
