use crate::propagator::*;
use crate::Real;
use rand_xorshift::XorShiftRng;
use rand::SeedableRng;
use crate::wave::WaveError;
use rand::distributions::*;

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

    /// Do a step of the WFC algorithm.
    /// This mean that we take the cell that has the lowest positive entropy,
    /// choose a pattern relative to the distribution, and propagate the information
    pub fn step(&mut self) -> Result<(), WaveError> {
        let (y, x) = self.propagator.wave().get_min_entropy()?;
        let weights = self.propagator.wave()[y][x].iter().zip(self.patterns_weights.iter()).map(|(b, w)| if *b {*w} else {0.0});
        let wc = WeightedIndex::new(weights).unwrap();

        // Choose a pattern fllowing the weight distribution
        let chosen_pattern = wc.sample(&mut self.rng_gen);

        for k in 0..self.patterns_weights.len() {
            if k != chosen_pattern {
                self.propagator.unset(y, x, k);
            }
        }

        Ok(())
    }
}
