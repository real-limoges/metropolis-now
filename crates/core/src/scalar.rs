//! Arithmetic for log density

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// So out of sheer laziness, I'm only implementing mixed arithmetic
/// in one direction. So it has to be (`x * 2.0`) otherwise
/// I have to do some annoying where's.
pub trait Scalar:
    Copy
    + Debug
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Add<f64, Output = Self>
    + Sub<f64, Output = Self>
    + Mul<f64, Output = Self>
    + Div<f64, Output = Self>
{
    /// constant, no derivative
    fn from_f64(x: f64) -> Self;

    /// use this for branching, not computing
    fn value(self) -> f64;

    fn exp(self) -> Self;
    fn ln(self) -> Self;
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
}

impl Scalar for f64 {
    fn from_f64(x: f64) -> Self {
        x
    }

    fn value(self) -> f64 {
        self
    }

    fn exp(self) -> Self {
        f64::exp(self)
    }

    fn ln(self) -> Self {
        f64::ln(self)
    }

    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    fn powi(self, n: i32) -> Self {
        f64::powi(self, n)
    }
}
