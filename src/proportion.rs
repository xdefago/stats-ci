//! Confidence intervals for proportions
//!
//! # Examples
//!
//! ```
//! # fn test() -> stats_ci::CIResult<()> {
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
//! * <https://influentialpoints.com/Training/confidence_intervals_of_proportions-principles-properties-assumptions.htm>
//!

use super::*;
use crate::stats::z_value;
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
/// # fn test() -> stats_ci::CIResult<()> {
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
/// # fn test() -> stats_ci::CIResult<()> {
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
/// # fn test() -> stats_ci::CIResult<()> {
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
/// Check if the conditions for the validity of the Wilson score interval are met.
/// The condetions for the validity of hypothesis tests (from which the Wilson score is derived) are stated as follows:
/// <https://www.itl.nist.gov/div898/handbook/prc/section2/prc24.htm>
/// 1. The sample size is large enough to ensure that the sampling distribution of the sample proportion is approximately normal (N > 30)
/// 2. The number of successes and failures are large enough to ensure that the sampling distribution of the sample proportion is approximately normal (x > 5 and n - x > 5)
///
/// # Arguments
///
/// * `population` - the size of the population
/// * `successes` - the number of successes in the sample
///
/// # Returns
///
/// `true` if the conditions are met, `false` otherwise.
///
/// # Examples
///
/// ```
/// use stats_ci::*;
/// assert!(proportion::is_significant(500, 10));
/// assert!(! proportion::is_significant(10, 5));
/// assert!(! proportion::is_significant(1000, 1));
/// ```
pub fn is_significant(population: usize, successes: usize) -> bool {
    // significance criteria for Wilson score intervals.
    // see https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Wilson_score_interval
    // The condetions for the validity of hypothesis tests (from which the Wilson score is derived) are stated as follows:
    // https://www.itl.nist.gov/div898/handbook/prc/section2/prc24.htm
    // 1. The sample size is large enough to ensure that the sampling distribution of the sample proportion is approximately normal (N > 30)
    (population > 30)
    // 2. The number of successes and failures are large enough to ensure that the sampling distribution of the sample proportion is approximately normal (x > 5 and n - x > 5)
    && (successes > 5)
    && (population - successes > 5)
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
/// Using the Wilson score interval, the probability of success \\( p \\) is estimated by:
/// \\[
/// p \approx  \frac{n_S+\frac{1}{2}z^2}{n+z^2} \pm \frac{z}{n+z^2} \sqrt{\frac{n_S ~ n_F}{n}+\frac{z^2}{4}}
/// \\]
/// where
/// * \\( n_S \\) is the number of successes,
/// * \\( n_F \\) is the number of failures,
/// * \\( n = n_S + n_F \\) is the sample size, and
/// * \\( z \\) is the z-value corresponding to the confidence level.
/// 
/// The conditions for the validity of the Wilson score interval can be checked with the function [`is_significant`].
/// However, the significance check for this function is much more permissive. It is the caller's responsibility to check for the stricter conditions for statistical significance if necessary.
/// One advantage of using the Wilson score interval is that it is still reasonably accurate for small sample sizes and when the proportion of successes is close to 0 or 1.
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
    // however, we are more permissive here and rely on the user to check for the stricter conditions for statistical significance.
    if successes < 2 {
        // too few successes for statistical significance
        return Err(CIError::TooFewSuccesses(successes, population, n_s));
    }
    if population - successes < 2 {
        // too few failures for statistical significance
        return Err(CIError::TooFewFailures(
            population - successes,
            population,
            n_f,
        ));
    }

    let z = z_value(confidence);
    let z_sq = z * z;

    let mean = (n_s + z_sq / 2.) / (n + z_sq);
    let span = (z / (n + z_sq)) * ((n_s * n_f / n) + (z_sq / 4.)).sqrt();

    match confidence {
        Confidence::TwoSided(_) => Interval::new(mean - span, mean + span).map_err(|e| e.into()),
        Confidence::UpperOneSided(_) => Interval::new(mean - span, 1.).map_err(|e| e.into()),
        Confidence::LowerOneSided(_) => Interval::new(0., mean + span).map_err(|e| e.into()),
    }
}

///
/// computes the (two sided) confidence interval over the proportion of successes a given sample using the Wilson score interval.
/// This is the method used by default when calling the function [`ci`] of this module.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `population` - the size of the population
/// * `success_rate` - the proportion of successes in the sample
///
/// # Errors
///
/// * `TooFewSuccesses` - if the number of successes is too small to compute a confidence interval
/// * `TooFewFailures` - if the number of failures is too small to compute a confidence interval
/// * `InvalidSuccesses` - if the number of successes is larger than the population size
/// * `NonPositiveValue` - if the success rate is not in positive or null.
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Notes
///
/// This method is simply a front for [`ci_wilson`], which takes the number of successes as an argument.
///
pub fn ci_wilson_ratio(
    confidence: Confidence,
    population: usize,
    success_rate: f64,
) -> CIResult<Interval<f64>> {
    if success_rate <= 0. {
        return Err(CIError::NonPositiveValue(success_rate));
    }
    let successes = (success_rate * population as f64) as usize;

    ci_wilson(confidence, population, successes)
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
    let mean = p;
    let span = z * std_dev;
    match confidence {
        Confidence::TwoSided(_) => Interval::new(mean - span, mean + span).map_err(|e| e.into()),
        Confidence::UpperOneSided(_) => Interval::new(mean - span, 1.).map_err(|e| e.into()),
        Confidence::LowerOneSided(_) => Interval::new(0., mean + span).map_err(|e| e.into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_proportion_ci() -> CIResult<()> {
        let population = 500;
        let successes = 421;
        let confidence = Confidence::TwoSided(0.95);
        let ci = proportion::ci(confidence, population, successes)?;
        assert_approx_eq!(ci.low_f(), 0.81, 1e-2);
        assert_approx_eq!(ci.high_f(), 0.87, 1e-2);

        let ci2 = proportion::ci(Confidence::UpperOneSided(0.975), population, successes)?;
        assert_eq!(ci2.high_f(), 1.);
        assert_approx_eq!(ci2.low_f(), ci.low_f(), 1e-2);

        let ci2 = proportion::ci(Confidence::LowerOneSided(0.975), population, successes)?;
        assert_eq!(ci2.low_f(), 0.);
        assert_approx_eq!(ci2.high_f(), ci.high_f(), 1e-2);

        Ok(())
    }

    #[test]
    fn test_proportion_ci_if() {
        let data = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let confidence = Confidence::TwoSided(0.95);
        let ci = proportion::ci_if(confidence, &data, |&x| x <= 10).unwrap();
        assert_approx_eq!(ci.low_f(), 0.299, 1e-2);
        assert_approx_eq!(ci.high_f(), 0.701, 1e-2);
    }
}
