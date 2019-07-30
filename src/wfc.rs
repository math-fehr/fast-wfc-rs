use crate::propagator::*;
use crate::utils::vec2d::*;
use crate::wave::WaveError;
use crate::Real;
use crate::direction::*;
use rand::distributions::*;
use rand::SeedableRng;
use rand_xorshift::XorShiftRng;

pub struct WFC {
    /// The random number generator
    rng_gen: XorShiftRng,
    /// The distribution of patterns
    patterns_weights: Vec<Real>,
    /// The propagator, that is used to propagate the information
    propagator: Propagator,
}

impl WFC {
    /// Create the object containing all the information to perform the WFC
    /// algorithm.
    pub fn new(
        is_toric: bool,
        seed: [u8; 16],
        patterns_weights: Vec<Real>,
        patterns_compatibility: Vec<DirArray<Vec<usize>>>,
        height: usize,
        width: usize,
    ) -> Self {
        let propagator = Propagator::new(
            height,
            width,
            patterns_weights.clone(),
            patterns_compatibility,
            is_toric,
        );
        WFC {
            rng_gen: XorShiftRng::from_seed(seed),
            patterns_weights,
            propagator,
        }
    }

    /// Restart WFC.
    pub fn restart(&mut self, seed: [u8; 16]) {
        self.propagator.reset();
        self.rng_gen = XorShiftRng::from_seed(seed);
    }

    /// Do steps of the wfc algorithm until completion
    /// Return true if the algorithm finished successfully,
    /// or false if the algorithm failed.
    pub fn run(&mut self) -> Option<Vec2D<usize>> {
        loop {
            let step_status = self.step();
            match step_status {
                Ok(()) => (),
                Err(WaveError::Impossible) => return None,
                Err(WaveError::Finished) => return self.to_output(),
            }
        }
    }

    /// Get the underlying propagator
    pub fn propagator(&mut self) -> &mut Propagator {
        &mut self.propagator
    }

    /// Do a step of the WFC algorithm.
    /// This mean that we take the cell that has the lowest positive entropy,
    /// choose a pattern relative to the distribution, and propagate the information
    pub fn step(&mut self) -> Result<(), WaveError> {
        let (y, x) = self.propagator.wave().get_min_entropy()?;
        let weights = self.propagator.wave()[(y,x)]
            .iter()
            .zip(self.patterns_weights.iter())
            .map(|(b, w)| if *b { *w } else { 0.0 });
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

    /// If every cell in the wave is decided, return the values decided in
    /// each cell.
    fn to_output(&self) -> Option<Vec2D<usize>> {
        let wave = self.propagator.wave();
        let height = wave.height();
        let width = wave.width();

        let mut data = Vec2D::new(height, width, &0);
        for i in 0..height {
            for j in 0..width {
                let cell_values: Vec<_> = wave[(i,j)]
                    .iter()
                    .enumerate()
                    .filter_map(|(v, b)| if *b { Some(v) } else { None })
                    .collect();
                if cell_values.len() != 1 {
                    return None;
                }
                data[i][j] = cell_values[0];
            }
        }
        Some(data)
    }
}
