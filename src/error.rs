pub type CIResult<T> = Result<T, CIError>;

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
    // wrapper errors
}
