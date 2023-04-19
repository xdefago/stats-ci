pub type CIResult<T> = Result<T, CIError>;

pub use crate::interval::IntervalError;

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

    #[error("Geometric mean requires strictly positive values: found {0}")]
    NonPositiveValue(f64),

    #[error("Invalid input data found")]
    InvalidInputData,

    #[error("Float type conversion error: {0}")]
    FloatConversionError(String),

    // wrapper errors
    #[error("String error: {0}")]
    Error(String),

    #[error("Interval error: {0}")]
    IntervalError(#[from] IntervalError),
}

impl From<&str> for CIError {
    fn from(s: &str) -> Self {
        CIError::Error(s.to_string())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("Empty interval has no concrete bounds")]
    NoConcreteBoundsError,

    #[error("Degenerate interval has single bound")]
    SingleBoundError,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_to_error() -> CIResult<()> {
        Err("This is a string error")?
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
