//! Contains the OverlappingWFC struct, which is used to apply the overlapping WFC on a 2D image

use crate::direction::*;
use crate::utils::vec2d::*;
use crate::wfc::WFC;
use std::collections::hash_map::{DefaultHasher, HashMap};
use std::hash::{BuildHasherDefault, Hash};

/// The available options used for overlappingWFC
#[derive(Clone, Copy, Debug)]
pub struct OverlappingWFCOptions {
    pub periodic_input: bool,
    pub periodic_output: bool,
    pub out_height: usize,
    pub out_width: usize,
    pub symmetry: usize,
    pub pattern_size: usize,
    pub ground: bool,
}

/// Class used for the overlapping WFC
pub struct OverlappingWFC<T> {
    options: OverlappingWFCOptions,
    wfc: WFC,
    patterns: Vec<Vec2D<T>>,
}

impl<T: Eq + Hash + Clone> OverlappingWFC<T> {
    pub fn new(
        input: Vec2D<T>,
        options: OverlappingWFCOptions,
        seed: [u8; 16],
    ) -> OverlappingWFC<T> {
        assert!(options.pattern_size <= options.out_height);
        assert!(options.pattern_size <= options.out_width);
        let patterns = get_patterns(
            &input,
            options.periodic_input,
            options.pattern_size,
            options.symmetry,
        );

        let (patterns, weights): (Vec<_>, _) =
            patterns.into_iter().map(|(p, w)| (p, w as f32)).unzip();
        let compatible = precompute_compatible(&patterns);

        let wfc = WFC::new(
            options.periodic_output,
            seed,
            weights,
            compatible,
            options.out_height,
            options.out_width,
        );

        let mut wfc = OverlappingWFC {
            options,
            wfc,
            patterns,
        };
        if options.ground {
            wfc.init_ground(&input);
        }
        wfc
    }

    /// Initialize the ground, given the ground pattern
    fn init_ground(&mut self, input: &Vec2D<T>) {
        let ground = get_ground_pattern(input, &self.options);
        let ground_id = self
            .patterns
            .iter()
            .enumerate()
            .find_map(|(i, x)| if *x == ground {Some(i)} else {None})
            .unwrap();

        let propagator = self.wfc.propagator();
        let height = propagator.wave().height();
        let width = propagator.wave().width();
        for j in 0..width {
            for p in 0..self.patterns.len() {
                if p != ground_id {
                    self.wfc.propagator().unset(height - 1, j, p);
                }
            }
        }

        for i in 0..height-1 {
            for j in 0..width {
                self.wfc.propagator().unset(i, j, ground_id);
            }
        }
    }

    /// Run the wfc algorithm with overlapping.
    pub fn run(&mut self) -> Option<Vec2D<T>> {
        self.wfc.run().map(|patterns| self.to_image(&patterns))
    }

    /// Return the result image, given the selected patterns for each cell.
    fn to_image(&self, output_patterns: &Vec2D<usize>) -> Vec2D<T> {
        let height = self.options.out_height;
        let width = self.options.out_width;
        let pattern_size = self.options.pattern_size;
        if self.options.periodic_output {
            Vec2D::from_generator(height, width, |i, j| {
                self.patterns[output_patterns[i][j]][0][0].clone()
            })
        } else {
            Vec2D::from_generator(height, width, |i, j| {
                let (i, di) = if i < pattern_size {
                    (0, i)
                } else {
                    (i - pattern_size + 1, pattern_size - 1)
                };
                let (j, dj) = if j < pattern_size {
                    (0, j)
                } else {
                    (j - pattern_size + 1, pattern_size - 1)
                };
                self.patterns[output_patterns[i][j]][di][dj].clone()
            })
        }
    }
}

/// Precompute the is_compatible function for a set of patterns.
fn precompute_compatible<T: PartialEq>(patterns: &[Vec2D<T>]) -> Vec<DirArray<Vec<usize>>> {
    patterns
        .iter()
        .map(|pattern1| {
            DirArray::new_generator(|direction| {
                patterns
                    .iter()
                    .enumerate()
                    .filter_map(|(id, pattern2)| {
                        if is_compatible(pattern1, pattern2, direction) {
                            Some(id)
                        } else {
                            None
                        }
                    })
                    .collect()
            })
        })
        .collect()
}

/// Check if pattern1 is compatible with pattern2, when pattern2 is the neighbor
/// in direction dir of pattern1.
fn is_compatible<T: PartialEq>(pattern1: &Vec2D<T>, pattern2: &Vec2D<T>, dir: Direction) -> bool {
    assert!(pattern1.width() == pattern2.width());
    assert!(pattern1.height() == pattern2.height());
    assert!(pattern1.height() >= 1);
    assert!(pattern1.width() >= 1);
    let (dy, dx) = dir.get_coordinates();
    let (x_min, x_max) = if dx < 0 {
        (0, (dx + pattern2.width() as isize) as usize)
    } else {
        (dx as usize, pattern1.width())
    };
    let (y_min, y_max) = if dy < 0 {
        (0, (dy + pattern2.height() as isize) as usize)
    } else {
        (dy as usize, pattern1.width())
    };

    for y in y_min..y_max {
        for x in x_min..x_max {
            if pattern1[y][x] != pattern2[(y as isize - dy) as usize][(x as isize - dx) as usize] {
                return false;
            }
        }
    }

    true
}

/// Get the list of patterns in the input, as well as the number of time they appear in the input.
pub fn get_patterns<T>(
    input: &Vec2D<T>,
    periodic: bool,
    pattern_size: usize,
    symmetry: usize,
) -> Vec<(Vec2D<T>, usize)>
where
    T: Clone + Hash + Eq,
{
    let mut patterns: HashMap<_, _, BuildHasherDefault<DefaultHasher>> = HashMap::default();

    let max_i = if periodic {
        input.height()
    } else {
        input.height() - pattern_size + 1
    };

    let max_j = if periodic {
        input.width()
    } else {
        input.width() - pattern_size + 1
    };

    for i in 0..max_i {
        for j in 0..max_j {
            let mut symmetries = Vec::new();
            let pattern = input.get_sub_vec(i, j, pattern_size, pattern_size);
            symmetries.push(pattern);

            // We only support symmetry of size 1, 2, 4 and 8
            if symmetry > 1 {
                symmetries.push(symmetries[0].reflected());
            }
            if symmetry > 2 {
                symmetries.push(symmetries[0].rotated());
                symmetries.push(symmetries[2].reflected());
            }
            if symmetry > 4 {
                symmetries.push(symmetries[2].rotated());
                symmetries.push(symmetries[4].reflected());
                symmetries.push(symmetries[4].rotated());
                symmetries.push(symmetries[6].reflected());
            }

            for symmetry in symmetries {
                let occurence = patterns.entry(symmetry).or_insert(0);
                *occurence += 1;
            }
        }
    }

    patterns.into_iter().collect()
}

/// Get the middle bottommost pattern of the input.
/// If the input is toric, then this pattern is the one having only one pixel
/// in the bottom, and options.pattern_size - 1 pixels in the top
pub fn get_ground_pattern<T: Clone>(input: &Vec2D<T>, options: &OverlappingWFCOptions) -> Vec2D<T> {
    if options.periodic_input {
        input.get_sub_vec(
            input.height() - 1,
            (input.width() - options.pattern_size) / 2,
            options.pattern_size,
            options.pattern_size,
        )
    } else {
        input.get_sub_vec(
            input.height() - options.pattern_size,
            (input.width() - options.pattern_size) / 2,
            options.pattern_size,
            options.pattern_size,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_patterns() {
        // 0 1 2
        // 3 4 5
        // 6 7 8
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let input = Vec2D::from_vec(input, 3, 3);

        let patterns = get_patterns(&input, false, 2, 1);
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![0, 1, 3, 4] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![1, 2, 4, 5] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![3, 4, 6, 7] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![4, 5, 7, 8] && *weight == 1
            )
            .is_some());

        assert_eq!(patterns.len(), 4);
    }

    #[test]
    fn test_get_patterns_overlapping() {
        // 0 1
        // 2 3
        let input = vec![0, 1, 2, 3];
        let input = Vec2D::from_vec(input, 2, 2);

        let patterns = get_patterns(&input, true, 2, 1);
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![0, 1, 2, 3] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![1, 0, 3, 2] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![2, 3, 0, 1] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![3, 2, 1, 0] && *weight == 1
            )
            .is_some());

        assert_eq!(patterns.len(), 4);
    }

    #[test]
    fn test_get_patterns_symmetry_2() {
        // 0 1
        // 2 3
        let input = vec![0, 1, 2, 3];
        let input = Vec2D::from_vec(input, 2, 2);

        let patterns = get_patterns(&input, false, 2, 2);
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![0, 1, 2, 3] && *weight == 1
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![1, 0, 3, 2] && *weight == 1
            )
            .is_some());

        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_get_patterns_multiple() {
        // 0 1 0
        // 1 0 1
        // 0 1 0
        let input = vec![0, 1, 0, 1, 0, 1, 0, 1, 0];
        let input = Vec2D::from_vec(input, 3, 3);

        let patterns = get_patterns(&input, false, 2, 1);
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![0, 1, 1, 0] && *weight == 2
            )
            .is_some());
        assert!(patterns
            .iter()
            .find(
                |(pattern, weight)| pattern.clone().into_vec() == vec![1, 0, 0, 1] && *weight == 2
            )
            .is_some());

        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_is_compatible_true() {
        // 1 2 3
        // 4 5 6
        // 7 8 9
        let pattern1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let pattern1 = Vec2D::from_vec(pattern1, 3, 3);

        // 2 3 0
        // 5 6 0
        // 8 9 0
        let pattern2 = vec![2, 3, 0, 5, 6, 0, 8, 9, 0];
        let pattern2 = Vec2D::from_vec(pattern2, 3, 3);

        assert!(is_compatible(&pattern1, &pattern2, Direction::Right))
    }

    #[test]
    fn test_is_compatible_false() {
        // 1 2 3
        // 4 5 6
        // 7 8 9
        let pattern1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let pattern1 = Vec2D::from_vec(pattern1, 3, 3);

        // 2 3 0
        // 5 0 0
        // 8 9 0
        let pattern2 = vec![2, 3, 0, 5, 0, 0, 8, 9, 0];
        let pattern2 = Vec2D::from_vec(pattern2, 3, 3);

        assert!(!is_compatible(&pattern1, &pattern2, Direction::Right))
    }

}
