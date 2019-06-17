//! Contains the OverlappingWFC struct, which is used to apply the overlapping WFC on a 2D image

use crate::utils::vec2d::*;
use std::collections::HashMap;
use std::hash::Hash;
use crate::Real;

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

/// Get the list of patterns in the input, as well as their probability of appearance.
fn get_patterns<T>(input: &Vec2D<T>, options: &OverlappingWFCOptions) -> Vec<(Vec2D<T>, Real)>
where T: Clone + Hash + Eq,
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

    patterns.into_iter().map(|(pattern, occurence)| (pattern, occurence as Real)).collect()
}
