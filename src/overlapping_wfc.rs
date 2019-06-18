//! Contains the OverlappingWFC struct, which is used to apply the overlapping WFC on a 2D image

use crate::utils::vec2d::*;
use std::collections::HashMap;
use std::hash::Hash;

/// The available options used for overlappingWFC
struct OverlappingWFCOptions {
    pub periodic_input: bool,
    pub periodic_output: bool,
    pub out_height: usize,
    pub out_width: usize,
    pub symmetry: usize,
    pub ground: bool,
    pub pattern_size: usize,
}

/// Class used for the overlapping WFC
struct OverlappingWFC<T> {
    input: Vec2D<T>,
    options: OverlappingWFCOptions,
}

/// Get the list of patterns in the input, as well as the number of time they appear in the input.
fn get_patterns<T>(input: &Vec2D<T>, options: &OverlappingWFCOptions) -> Vec<(Vec2D<T>, usize)>
where
    T: Clone + Hash + Eq,
{
    let mut patterns = HashMap::new();

    let max_i = if options.periodic_input {
        input.height()
    } else {
        input.height() - options.pattern_size + 1
    };

    let max_j = if options.periodic_input {
        input.width()
    } else {
        input.width() - options.pattern_size + 1
    };

    for i in 0..max_i {
        for j in 0..max_j {
            let mut symmetries = Vec::new();
            let pattern = input.get_sub_vec(i, j, options.pattern_size, options.pattern_size);
            symmetries.push(pattern);

            // We only support symmetry of size 1, 2, 4 and 8
            if options.symmetry > 1 {
                symmetries.push(symmetries[0].reflected());
            }
            if options.symmetry > 2 {
                symmetries.push(symmetries[0].rotated());
                symmetries.push(symmetries[2].reflected());
            }
            if options.symmetry > 4 {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_patterns() {
        let options = OverlappingWFCOptions {
            periodic_input: false,
            periodic_output: false,
            out_height: 10,
            out_width: 10,
            symmetry: 1,
            ground: false,
            pattern_size: 2,
        };

        // 0 1 2
        // 3 4 5
        // 6 7 8
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let input = Vec2D::from_vec(3, 3, input);

        let patterns = get_patterns(&input, &options);
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![0, 1, 3, 4] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![1, 2, 4, 5] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![3, 4, 6, 7] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![4, 5, 7, 8] && *weight == 1)
            .is_some());

        assert_eq!(patterns.len(), 4);
    }

    #[test]
    fn test_get_patterns_overlapping() {
        let options = OverlappingWFCOptions {
            periodic_input: true,
            periodic_output: false,
            out_height: 10,
            out_width: 10,
            symmetry: 1,
            ground: false,
            pattern_size: 2,
        };

        // 0 1
        // 2 3
        let input = vec![0, 1, 2, 3];
        let input = Vec2D::from_vec(2, 2, input);

        let patterns = get_patterns(&input, &options);
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![0, 1, 2, 3] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![1, 0, 3, 2] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![2, 3, 0, 1] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![3, 2, 1, 0] && *weight == 1)
            .is_some());

        assert_eq!(patterns.len(), 4);
    }

    #[test]
    fn test_get_patterns_symmetry_2() {
        let options = OverlappingWFCOptions {
            periodic_input: false,
            periodic_output: false,
            out_height: 10,
            out_width: 10,
            symmetry: 2,
            ground: false,
            pattern_size: 2,
        };

        // 0 1
        // 2 3
        let input = vec![0, 1, 2, 3];
        let input = Vec2D::from_vec(2, 2, input);

        let patterns = get_patterns(&input, &options);
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![0, 1, 2, 3] && *weight == 1)
            .is_some());
        assert!(patterns
            .iter()
            .find(|(pattern, weight)| pattern.clone().to_vec() == vec![1, 0, 3, 2] && *weight == 1)
            .is_some());

        assert_eq!(patterns.len(), 2);
    }

    #[test]
    fn test_get_patterns_multiple() {
        let options = OverlappingWFCOptions {
            periodic_input: false,
            periodic_output: false,
            out_height: 10,
            out_width: 10,
            symmetry: 1,
            ground: false,
            pattern_size: 2,
        };

        // 0 1 0
        // 1 0 1
        // 0 1 0
        let input = vec![0, 1, 0, 1, 0, 1, 0, 1, 0];
        let input = Vec2D::from_vec(3, 3, input);

        let patterns = get_patterns(&input, &options);
        assert!(patterns
                .iter()
                .find(|(pattern, weight)| pattern.clone().to_vec() == vec![0, 1, 1, 0] && *weight == 2)
                .is_some());
        assert!(patterns
                .iter()
                .find(|(pattern, weight)| pattern.clone().to_vec() == vec![1, 0, 0, 1] && *weight == 2)
                .is_some());

        assert_eq!(patterns.len(), 2);
    }
}