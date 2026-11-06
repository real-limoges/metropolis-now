//!

use crate::scalar::Scalar;

pub trait Dim {
    fn dim(&self) -> usize;
}

/// what people care about
pub trait Model: Dim {
    fn log_density<S: Scalar>(&self, q: &[S]) -> S;
}

/// what gradient-free samplers need
pub trait LogDensity: Dim {
    /// this is allowed to be `-inf` if outside the support
    /// Never allowed to be `NaN`
    fn log_prob(&self, q: &[f64]) -> f64;
}

/// what gradient samplers need
pub trait LogDensityGrad: LogDensity {
    fn log_prob_grad(&self, q: &[f64], grad: &mut [f64]) -> f64;
}

impl<M: Model> LogDensity for M {
    fn log_prob(&self, q: &[f64]) -> f64 {
        self.log_density(q)
    }
}
