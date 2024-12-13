//!
//! Error types and conversion traits
//!
//! The crate defines a type [`CIError`] to represent errors that can occur during the computation of confidence intervals.
//! The type [`CIResult<T>`] is a type alias for [`Result<T, CIError>`].
//!

///
/// Result type for confidence interval computations.
///
/// This type is a type alias for [`Result<T, CIError>`].
///  
pub type CIResult<T> = Result<T, CIError>;

use num_traits::Float;

///
/// Error types for confidence interval computations.
///
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum CIError {
    #[error("Too few samples to compute: {0}")]
    TooFewSamples(usize),

    #[error("Too few successes: {0} (population: {1}; n*p={2}))")]
    TooFewSuccesses(usize, usize, f64),

    #[error("Too few failures: {0} (population: {1}; n*q={2}))")]
    TooFewFailures(usize, usize, f64),

    #[error("Invalid confidence level (must be )): {0}")]
    InvalidConfidenceLevel(f64),

    #[error("Invalid quantile (must be in (0, 1)): {0}")]
    InvalidQuantile(f64),

    #[error("Invalid number of successes: {0} (population: {1})")]
    InvalidSuccesses(usize, usize),

    #[error("Geometric/harmonic mean require strictly positive values: found {0}")]
    NonPositiveValue(f64),

    #[error("Invalid input data found")]
    InvalidInputData,

    #[cfg(not(feature = "std"))]
    #[error("Float type conversion error: {0}")]
    FloatConversionError(&'static str),
    #[cfg(feature = "std")]
    #[error("Float type conversion error: {0}")]
    FloatConversionError(String),

    #[error("Index error: {0} should be in [0, {1})")]
    IndexError(f64, usize),

    // #[cfg(not(feature="std"))]
    // // wrapper errors
    // #[error("String error: {0}")]
    // Error(String),
    #[cfg(feature = "std")]
    #[error("String error: {0}")]
    Error(String),

    #[error("Interval error: {0}")]
    IntervalError(#[from] IntervalError),

    #[error("Different sample sizes: {0} vs. {1}")]
    DifferentSampleSizes(usize, usize),
}

///
/// An error type for interval creation.
///
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum IntervalError {
    #[error("Invalid bounds: the left bound is greater than the right bound")]
    InvalidBounds,

    #[error("Empty interval")]
    EmptyInterval,
}

///
/// Error types for conversion from a generic [`Float`] type to a [`CIResult<f64>`].
///
#[allow(missing_docs)]
#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("Empty interval has no concrete bounds")]
    NoConcreteBoundsError,

    #[error("Degenerate interval has single bound")]
    SingleBoundError,
}

///
/// Decorator trait used to convert from a generic [`Float`] type to a [`CIResult<f64>`]
///
pub(crate) trait FloatConversion<F: Float> {
    fn try_f64(&self, var_name: &str) -> CIResult<f64>;
}

impl<F: Float> FloatConversion<F> for F {
    #[cfg(feature = "std")]
    #[inline]
    fn try_f64(&self, var_name: &str) -> CIResult<f64> {
        self.to_f64().ok_or_else(|| {
            CIError::FloatConversionError(format!(
                "Error converting {} ({}) to f64",
                var_name,
                std::any::type_name::<F>()
            ))
        })
    }

    #[cfg(not(feature = "std"))]
    #[inline]
    fn try_f64(&self, _var_name: &str) -> CIResult<f64> {
        self.to_f64()
            .ok_or_else(|| CIError::FloatConversionError("Error converting to f64"))
    }
}

///
/// Decorator trait used to convert from an [`Option<F>`] to a [`CIResult<F>`]
///
pub(crate) trait FloatReverseConversion<F: num_traits::Float> {
    fn convert(&self, var_name: &str) -> CIResult<F>;
}

impl<F: Float> FloatReverseConversion<F> for Option<F> {
    #[cfg(feature = "std")]
    #[inline]
    fn convert(&self, var_name: &str) -> CIResult<F> {
        self.ok_or_else(|| {
            CIError::FloatConversionError(format!(
                "Error converting {} to {}",
                var_name,
                std::any::type_name::<F>()
            ))
        })
    }

    #[cfg(not(feature = "std"))]
    #[inline]
    fn convert(&self, _var_name: &str) -> CIResult<F> {
        self.ok_or_else(|| CIError::FloatConversionError("Error converting Option to CIResult"))
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;

    fn string_to_error() -> CIResult<()> {
        Err(CIError::Error("This is a string error".to_string()))?
    }

    #[test]
    fn test_string_to_error() {
        let err = string_to_error();
        assert!(err.is_err());
        match err {
            Err(CIError::Error(s)) => assert_eq!(s, "This is a string error"),
            Err(e) => panic!("Unexpected error type: {:?}", e),
            Ok(_) => panic!("Unexpected success"),
        }
    }
}
