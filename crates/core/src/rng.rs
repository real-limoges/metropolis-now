//! Randomness, one stream per chain

/// ChaCha8 because this is not cryptography
use rand_chacha::ChaCha8Rng;
use rand_core::{RngCore, SeedableRng};

/// Built from a run's seed + chain's index. Every index gets its
/// own stream from ChaCha. No constructor without a seed.
pub struct ChainRng {
    inner: ChaCha8Rng,
}

impl ChainRng {
    pub fn new(seed: u64, chain: u64) -> Self {
        let mut inner = ChaCha8Rng::seed_from_u64(seed);
        inner.set_stream(chain);
        ChainRng { inner }
    }

    /// Grabs top 53 bits of `u64`, which matches standard f64 impl
    pub fn uniform(&mut self) -> f64 {
        (self.inner.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Box-Muller makes two normals per pair of uniforms. I keep
    /// one and throw the other away, which keeps `ChainRng`
    /// totally free of cached state.
    pub fn normal(&mut self) -> f64 {
        let u1 = 1.0 - self.uniform(); // (0, 1]
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    }
}
