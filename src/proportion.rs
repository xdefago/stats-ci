//! Confidence intervals for proportions
//!
//! # Examples
//!
//! ```
//! # fn test() -> Result<(), stats_ci::error::CIError> {
//! use stats_ci::*;
//!
//! let data = [
//!     true, false, true, true, false, true, true, false, true, true,
//!     false, false, false, true, false, true, false, false, true, false
//! ];
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci_true(confidence, data)?;
//! use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
//! # Ok(())
//! # }
//! ```
//!
//! # References
//!
//! * [Wikipedia - Confidence interval](https://en.wikipedia.org/wiki/Confidence_interval)
//! * [Wikipedia - Binomial proportion confidence interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval)
//! * https://influentialpoints.com/Training/confidence_intervals_of_proportions-principles-properties-assumptions.htm
//!

use super::*;
use error::*;

///
/// computes the (two sided) confidence interval over the proportion of true values in a given sample.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `data` - the sample
///
/// # Errors
///
/// * `TooFewSuccesses` - if the number of successes is too small to compute a confidence interval
/// * `TooFewFailures` - if the number of failures is too small to compute a confidence interval
/// * `InvalidSuccesses` - if the number of successes is larger than the population size
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Examples
///
/// ```
/// # fn test() -> Result<(), stats_ci::error::CIError> {
/// use stats_ci::*;
/// use assert_approx_eq::assert_approx_eq;
///
/// let data = [
///     true, false, true, true, false, true, true, false, true, true,
///     false, false, false, true, false, true, false, false, true, false
/// ];
/// let confidence = Confidence::new_two_sided(0.95);
/// let interval = proportion::ci_true(confidence, data)?;
/// assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
/// assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
/// # Ok(())
/// # }
/// ```
///
pub fn ci_true<T: IntoIterator<Item = bool>>(
    confidence: Confidence,
    data: T,
) -> CIResult<Interval<f64>> {
    let mut population = 0;
    let mut successes = 0;
    for x in data {
        population += 1;
        if x {
            successes += 1;
        }
    }
    ci(confidence, population, successes)
}

///
/// computes the (two sided) confidence interval over the proportion of a given sample that satisfies a given condition.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `population` - the size of the population
/// * `successes` - the number of successes in the sample
///
/// # Errors
///
/// * `TooFewSuccesses` - if the number of successes is too small to compute a confidence interval
/// * `TooFewFailures` - if the number of failures is too small to compute a confidence interval
/// * `InvalidSuccesses` - if the number of successes is larger than the population size
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Examples
///
/// ```
/// # fn test() -> Result<(), stats_ci::error::CIError> {
/// use stats_ci::*;
/// use assert_approx_eq::assert_approx_eq;
///
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
/// let confidence = Confidence::new_two_sided(0.95);
/// let interval = proportion::ci_if(confidence, &data, |&x| x <= 10)?;
/// assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
/// assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
/// # Ok(())
/// # }
/// ```
///
pub fn ci_if<T, I: IntoIterator<Item = T>, F: Fn(T) -> bool>(
    confidence: Confidence,
    data: I,
    cond: F,
) -> CIResult<Interval<f64>> {
    ci_true(confidence, data.into_iter().map(cond))
}

///
/// computes the (two sided) confidence interval over the proportion of successes a given sample.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `population` - the size of the population
/// * `successes` - the number of successes in the sample
///
/// # Errors
///
/// * `TooFewSuccesses` - if the number of successes is too small to compute a confidence interval
/// * `TooFewFailures` - if the number of failures is too small to compute a confidence interval
/// * `InvalidSuccesses` - if the number of successes is larger than the population size
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Notes
///
/// This function is an alias for [`ci_wilson`].
///
/// # Examples
///
/// ```
/// # fn test() -> Result<(), stats_ci::error::CIError> {
/// use stats_ci::*;
/// use assert_approx_eq::assert_approx_eq;
///
/// let population = 500;
/// let successes = 421;
/// let confidence = Confidence::new_two_sided(0.95);
/// let interval = proportion::ci(confidence, population, successes)?;
/// assert_approx_eq!(interval.low().unwrap(), 0.81, 1e-2);
/// assert_approx_eq!(interval.high().unwrap(), 0.87, 1e-2);
/// # Ok(())
/// # }
/// ```
///
pub fn ci(confidence: Confidence, population: usize, successes: usize) -> CIResult<Interval<f64>> {
    ci_wilson(confidence, population, successes)
}

///
/// computes the (two sided) confidence interval over the proportion of successes a given sample using the Wilson score interval.
/// This is the method used by default when calling the function [`ci`] of this module.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `population` - the size of the population
/// * `successes` - the number of successes in the sample
///
/// # Errors
///
/// * `TooFewSuccesses` - if the number of successes is too small to compute a confidence interval
/// * `TooFewFailures` - if the number of failures is too small to compute a confidence interval
/// * `InvalidSuccesses` - if the number of successes is larger than the population size
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Notes
///
/// This method is based on the Wilson score interval, which is a modification of the normal approximation interval.
/// It is more robust than the normal approximation interval, but it is also more conservative.
/// In particular, it is more conservative when the sample size is small.
/// It is also more conservative when the sample size is large and the proportion is close to 0 or 1.
///
/// # References
///
/// * [Wikipedia article on Wilson score interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval)
/// * Francis J. DiTraglia. [Blog post: The Wilson Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-wilson-confidence-interval-for-a-proportion/)
///
pub fn ci_wilson(
    confidence: Confidence,
    population: usize,
    successes: usize,
) -> CIResult<Interval<f64>> {
    if successes > population {
        return Err(CIError::InvalidSuccesses(successes, population));
    }

    let n = population as f64;
    let n_s = successes as f64;
    let n_f = n - n_s;

    // conditions for statistical significance:
    // n p > 5 and n (1 - p) > 5
    if successes <= 5 { 
        // too few successes for statistical significance
        return Err(CIError::TooFewSuccesses(successes, population, n_s));
    }
    if population - successes <= 5 {
        // too few failures for statistical significance
        return Err(CIError::TooFewFailures(
            population - successes,
            population,
            n_f,
        ));
    }

    let z = z_value(confidence);
    let z_2 = z * z;

    let mean = (n_s + z_2 / 2.) / (n + z_2);
    let span = (z / (n + z_2)) * ((n_s * n_f / n) + (z_2 / 4.)).sqrt();

    Ok(Interval::new(mean - span, mean + span))
}

///
/// computes the confidence interval over the proportion of successes a given sample using the normal approximation interval (Wald interval).
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `population` - the size of the population
/// * `successes` - the number of successes in the sample
///
/// # Errors
///
/// * `TooFewSuccesses` - if the number of successes is too small to compute a confidence interval
/// * `TooFewFailures` - if the number of failures is too small to compute a confidence interval
/// * `InvalidSuccesses` - if the number of successes is larger than the population size
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Notes
///
/// This method is based on the normal approximation interval.
/// It is less robust than the Wilson score interval, but it is also less conservative.
///
/// # References
///
/// * [Wikipedia article on normal approximation interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval)
/// * Francis J. DiTraglia. [Blog post: The Normal Approximation Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-normal-approximation-confidence-interval-for-a-proportion/)
///
pub fn ci_z_normal(
    confidence: Confidence,
    population: usize,
    successes: usize,
) -> CIResult<Interval<f64>> {
    if successes > population {
        return Err(CIError::InvalidSuccesses(successes, population));
    }

    let n = population as f64;
    let x = successes as f64;
    let p = x / n;
    let q = 1. - p;

    if n * p < 10. {
        // too few successes for statistical significance
        return Err(CIError::TooFewSuccesses(successes, population, n * p));
    }
    if n * q < 10. {
        // too few failures for statistical significance
        return Err(CIError::TooFewFailures(
            population - successes,
            population,
            n * q,
        ));
    }

    let std_dev = (p * q / n).sqrt();
    let z = z_value(confidence);
    Ok(Interval::new(p - z * std_dev, p + z * std_dev))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_proportion_ci() {
        let population = 500;
        let successes = 421;
        let confidence = Confidence::TwoSided(0.95);
        let ci = ci(confidence, population, successes).unwrap();
        assert_approx_eq!(ci.low().unwrap(), 0.81, 1e-2);
        assert_approx_eq!(ci.high().unwrap(), 0.87, 1e-2);
    }

    #[test]
    fn test_proportion_ci_if() {
        let data = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let confidence = Confidence::TwoSided(0.95);
        let ci = ci_if(confidence, &data, |&x| x <= 10).unwrap();
        assert_approx_eq!(ci.low().unwrap(), 0.299, 1e-2);
        assert_approx_eq!(ci.high().unwrap(), 0.701, 1e-2);
    }
}
