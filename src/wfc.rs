use crate::propagator::*;
use crate::Real;
use rand_xorshift::XorShiftRng;
use rand::SeedableRng;

pub struct WFC {
    /// The random number generator
    rng_gen: XorShiftRng,
    /// The distribution of patterns
    patterns_weights: Vec<Real>,
    /// The propagator, that is used to propagate the information
    propagator: Propagator,
}

impl WFC {
    pub fn new(
        is_toric: bool,
        seed: [u8; 16],
        patterns_weights: Vec<Real>,
        patterns_compatibility: Vec<[Vec<usize>; 4]>,
        width: usize,
        height: usize,
    ) -> Self {
        let propagator = Propagator::new(
            height,
            width,
            &patterns_weights,
            patterns_compatibility,
            is_toric,
        );
        WFC {
            rng_gen: XorShiftRng::from_seed(seed),
            patterns_weights,
            propagator,
        }
    }
}
