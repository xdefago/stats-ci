//!
//! Confidence intervals for proportions
//!
//! # Examples
//!
//! ```
//! use stats_ci::*;
//! let data = [
//!     true, false, true, true, false, true, true, false, true, true,
//!     false, false, false, true, false, true, false, false, true, false
//! ];
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci_true(confidence, data)?;
//! use approx::*;
//! assert_abs_diff_eq!(interval, Interval::new(0.299, 0.701)?, epsilon = 1e-2);
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! The confidence interval can also be computed incrementally, as follows:
//! ```
//! # use stats_ci::*;
//! # let data = [
//! #     true, false, true, true, false, true, true, false, true, true,
//! #     false, false, false, true, false, true, false, false, true, false
//! # ];
//! # let confidence = Confidence::new_two_sided(0.95);
//! let mut stats = proportion::Stats::default();
//! stats.extend(data);
//! let interval = stats.ci(confidence)?;
//! # use approx::*;
//! assert_abs_diff_eq!(interval, Interval::new(0.299, 0.701)?, epsilon = 1e-2);
//! # Ok::<(),error::CIError>(())
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
/// Represents the state of the computation of a confidence interval for a proportion.
///
/// # Examples
///
/// ```
/// # use stats_ci::*;
/// let grades = [40, 59, 73, 44, 82, 44, 58, 74, 94, 79, 40, 52, 100, 57, 76, 93, 68, 96, 92, 98, 58, 64, 76, 40, 89, 65, 63, 90, 66, 89];
/// let stats = proportion::Stats::from_iter(grades.iter().map(|&x| x >= 60));
/// let confidence = Confidence::new_two_sided(0.95);
/// let pass_rate_ci = stats.ci(confidence)?;
/// println!("Pass rate: {}", pass_rate_ci);
/// # use approx::*;
/// assert_abs_diff_eq!(pass_rate_ci, Interval::new(0.4878, 0.8077)?, epsilon = 1e-3);
/// # Ok::<(),error::CIError>(())
/// ```
///
/// # Panics
///
/// * if the number of successes is larger than the population size
///
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stats {
    population: usize,
    successes: usize,
}

impl FromIterator<bool> for Stats {
    ///
    /// Creates a new statistics object with initial values from a Boolean iterator counting the number of successes.
    ///
    /// Complexity: \\( O(n) \\) where \\( n \\) is the number of samples in `iter`.
    ///
    /// # Arguments
    ///
    /// * `iter` - a Boolean iterator or slice
    ///
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = bool>,
    {
        let mut stats = Stats::default();
        for value in iter {
            if value {
                stats.add_success();
            } else {
                stats.add_failure();
            }
        }
        stats
    }
}

impl Stats {
    ///
    /// Creates a new statistics object with initial values for the population size and the number of successes.
    /// The number of successes must not be larger than the population size.
    ///
    /// Complexity: \\( O(1) \\)
    ///
    /// # Panics
    ///
    /// * if the number of successes is larger than the population size
    ///
    pub const fn new(population: usize, successes: usize) -> Self {
        if population < successes {
            panic!("Number of successes must not be larger than population size.")
        }
        Stats {
            population,
            successes,
        }
    }

    ///
    /// Returns the population size (total number of samples).
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn population(&self) -> usize {
        self.population
    }

    ///
    /// Returns the number of successes (number of `true` values found in the sample).
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn successes(&self) -> usize {
        self.successes
    }

    ///
    /// Add a success to the statistics and updates the population accordingly.
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn add_success(&mut self) {
        self.population += 1;
        self.successes += 1;
    }

    ///
    /// Add a failure to the statistics and updates the population accordingly.
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn add_failure(&mut self) {
        self.population += 1;
    }

    ///
    /// Tests if the conditions for the validity of the Wilson score interval are met.
    /// The conditions for the validity of the Wilson score interval are stated as follows:
    /// <https://www.itl.nist.gov/div898/handbook/prc/section2/prc24.htm>
    /// 1. The sample size is large enough to ensure that the sampling distribution of the sample proportion is approximately normal (N > 30)
    /// 2. The number of successes and failures are large enough to ensure that the sampling distribution of the sample proportion is approximately normal (x > 5 and n - x > 5)
    pub fn is_significant(&self) -> bool {
        is_significant(self.population, self.successes)
    }

    /// Computes the confidence interval over the proportion of true values in a given sample.
    ///
    /// Complexity: \\( O(1) \\)
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level (must be in (0, 1))
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
    /// # use stats_ci::*;
    /// # use approx::*;
    /// let data = [
    ///    true, false, true, true, false, true, true, false, true, true,
    ///   false, false, false, true, false, true, false, false, true, false
    /// ];
    /// let confidence = Confidence::new_two_sided(0.95);
    /// let stats = proportion::Stats::from_iter(data);
    /// let interval = stats.ci(confidence)?;
    /// assert_abs_diff_eq!(interval, Interval::new(0.299, 0.701)?, epsilon = 1e-2);
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    /// # Notes
    ///
    /// The confidence interval is computed using the function [`ci_wilson`] (Wilson score interval).
    ///
    pub fn ci(&self, confidence: Confidence) -> CIResult<Interval<f64>> {
        ci(confidence, self.population, self.successes)
    }

    ///
    /// Extend the data with additional sample data.
    ///
    /// Complexity: \\( O(n) \\) where \\( n \\) is the number of samples in `data`.
    ///
    /// # Arguments
    ///
    /// * `data` - the sample given as a boolean iterator or slice
    ///
    /// # Examples
    /// ```
    /// # use stats_ci::*;
    /// let data = [true, false, true, true, false, true, true, false, true, true];
    /// let mut stats = proportion::Stats::default();
    /// stats.extend(data);
    /// assert_eq!(stats, proportion::Stats::new(10, 7));
    /// ```
    pub fn extend<I: IntoIterator<Item = bool>>(&mut self, data: I) {
        self.extend_if(data, |x| x)
    }

    ///
    /// Extend the data with additional sample data and a condition that must be satisfied to be counted as a success.
    ///
    /// Complexity: \\( O(n) \\) where \\( n \\) is the number of samples in `data`.
    ///
    /// # Arguments
    ///
    /// * `data` - the sample given as an iterator or slice
    /// * `is_success` - a function that returns `true` if a sample value is a success
    ///
    /// # Examples
    /// ```
    /// # use stats_ci::*;
    /// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let mut stats = proportion::Stats::default();
    /// stats.extend_if(data.iter(), |&x| x <= 5);
    /// assert_eq!(stats, proportion::Stats::new(10, 5));
    /// ```
    pub fn extend_if<T, I, F>(&mut self, data: I, is_success: F)
    where
        I: IntoIterator<Item = T>,
        F: Fn(T) -> bool,
    {
        for x_i in data {
            if is_success(x_i) {
                self.add_success();
            } else {
                self.add_failure();
            }
        }
    }
}

impl std::ops::Add for Stats {
    type Output = Self;

    ///
    /// Combines two statistics objects by adding the number of samples and the number of successes.
    ///
    /// Complexity: \\( O(1) \\)
    ///
    /// # Examples
    /// ```
    /// # use stats_ci::*;
    /// let stats1 = proportion::Stats::new(100, 50);
    /// let stats2 = proportion::Stats::new(200, 100);
    /// let stats = stats1 + stats2;
    /// assert_eq!(stats, proportion::Stats::new(300, 150));
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        Stats {
            population: self.population + rhs.population,
            successes: self.successes + rhs.successes,
        }
    }
}

///
/// Computes the (two sided) confidence interval over the proportion of true values in a given sample.
///
/// Complexity: \\( O(n) \\) where \\( n \\) is the number of samples in `data`.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `data` - the sample given as a boolean iterator or slice
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
/// use stats_ci::*;
/// # use approx::*;
/// let data = [
///     true, false, true, true, false, true, true, false, true, true,
///     false, false, false, true, false, true, false, false, true, false
/// ];
/// let confidence = Confidence::new_two_sided(0.95);
/// let interval = proportion::ci_true(confidence, data)?;
/// assert_abs_diff_eq!(interval, Interval::new(0.299, 0.701)?, epsilon = 1e-2);
/// # Ok::<(),error::CIError>(())
/// ```
///
/// # Notes
///
/// The confidence interval is computed using the function [`ci_wilson`] (Wilson score interval).
///
pub fn ci_true<T: IntoIterator<Item = bool>>(
    confidence: Confidence,
    data: T,
) -> CIResult<Interval<f64>> {
    let mut stats = Stats::default();
    stats.extend(data);
    stats.ci(confidence)
}

///
/// Computes the (two sided) confidence interval over the proportion of a given sample that satisfies a given condition.
///
/// Complexity: \\( O(n) \\) where \\( n \\) is the number of samples in `data`.
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `data` - the sample given as a boolean iterator or slice
/// * `condition` - the condition that must be satisfied to be counted as a success
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
/// use stats_ci::*;
/// # use approx::*;
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
/// let confidence = Confidence::new_two_sided(0.95);
/// let interval = proportion::ci_if(confidence, data, |x| x <= 10)?;
/// assert_abs_diff_eq!(interval, Interval::new(0.299, 0.701)?, epsilon = 1e-2);
/// # Ok::<(),error::CIError>(())
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
/// Computes the (two sided) confidence interval over the proportion of successes a given sample.
///
/// Complexity: \\( O(1) \\)
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
/// use stats_ci::*;
/// # use approx::*;
/// let population = 500;
/// let successes = 421;
/// let confidence = Confidence::new_two_sided(0.95);
/// let interval = proportion::ci(confidence, population, successes)?;
/// assert_abs_diff_eq!(interval, Interval::new(0.81, 0.87)?, epsilon = 1e-2);
/// # Ok::<(),error::CIError>(())
/// ```
///
pub fn ci(confidence: Confidence, population: usize, successes: usize) -> CIResult<Interval<f64>> {
    ci_wilson(confidence, population, successes)
}

///
/// Check if the conditions for the validity of the Wilson score interval are met.
/// The conditions for the validity of hypothesis tests (from which the Wilson score is derived) are stated as follows:
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
    // The conditions for the validity of hypothesis tests (from which the Wilson score is derived) are stated as follows:
    // https://www.itl.nist.gov/div898/handbook/prc/section2/prc24.htm
    // 1. The sample size is large enough to ensure that the sampling distribution of the sample proportion is approximately normal (N > 30)
    (population > 30)
    // 2. The number of successes and failures are large enough to ensure that the sampling distribution of the sample proportion is approximately normal (x > 5 and n - x > 5)
    && (successes > 5)
    && (population - successes > 5)
}

///
/// Computes the (two sided) confidence interval over the proportion of successes a given sample using the Wilson score interval.
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
/// * Francis J. DiTraglia. [Blog post: The Wilson Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-wilson-confidence-interval-for-a-proportion/). Feb 2022.
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
/// Computes the (two sided) confidence interval over the proportion of successes in a given sample using the Wilson score interval.
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
/// Computes the confidence interval over the proportion of successes in a given sample using the normal approximation interval (Wald interval).
///
/// Using the normal approximation interval (Wald method), the probability of success \\( p \\) is estimated by:
/// \\[
/// p \approx  \frac{n_S}{n} \pm z \sqrt{\frac{n_S ~ n_F}{n^3}} = \hat{p} \pm z \sqrt{\frac{\hat{p} ~ (1 - \hat{p})}{n}}
/// \\]
/// where
/// * \\( n_S \\) is the number of successes,
/// * \\( n_F \\) is the number of failures,
/// * \\( n = n_S + n_F \\) is the sample size,
/// * \\( z \\) is the z-value corresponding to the confidence level, and
/// * \\( \hat{p} = \frac{n_S}{n} \\) is the estimated probability of success.
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
    use approx::*;

    #[test]
    fn test_proportion_ci() -> CIResult<()> {
        let population = 500;
        let successes = 421;
        let confidence = Confidence::TwoSided(0.95);
        let ci = proportion::ci(confidence, population, successes)?;
        assert_abs_diff_eq!(ci, Interval::new(0.81, 0.87)?, epsilon = 1e-2);

        let ci2 = proportion::ci(Confidence::UpperOneSided(0.975), population, successes)?;
        assert_eq!(ci2.high_f(), 1.);
        assert_abs_diff_eq!(ci2.low_f(), ci.low_f(), epsilon = 1e-2);

        let ci2 = proportion::ci(Confidence::LowerOneSided(0.975), population, successes)?;
        assert_eq!(ci2.low_f(), 0.);
        assert_abs_diff_eq!(ci2.high_f(), ci.high_f(), epsilon = 1e-2);

        Ok(())
    }

    #[test]
    fn test_proportion_ci_if() {
        let data = [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ];
        let confidence = Confidence::TwoSided(0.95);
        let ci = proportion::ci_if(confidence, &data, |&x| x <= 10).unwrap();
        assert_abs_diff_eq!(ci, Interval::new(0.299, 0.701).unwrap(), epsilon = 1e-2);
    }

    #[test]
    fn test_main_example() -> CIResult<()> {
        let grades = [
            40, 59, 73, 44, 82, 44, 58, 74, 94, 79, 40, 52, 100, 57, 76, 93, 68, 96, 92, 98, 58,
            64, 76, 40, 89, 65, 63, 90, 66, 89,
        ];
        let stats = proportion::Stats::from_iter(grades.iter().map(|&x| x >= 60));
        let confidence = Confidence::new_two_sided(0.95);
        let pass_rate_ci = stats.ci(confidence)?;
        println!("Pass rate: {}", pass_rate_ci);
        use approx::*;
        assert_abs_diff_eq!(pass_rate_ci, Interval::new(0.4878, 0.8077)?, epsilon = 1e-3);
        Ok(())
    }

    #[test]
    fn test_readme_simple() {
        let confidence = Confidence::new(0.95);
        let messages = 10_000;
        let losses = 89;
        let ci = proportion::ci(confidence, messages, losses).unwrap();
        println!("Loss rate: {}", ci);

        let confidence = Confidence::new_lower(0.95);
        let ci = proportion::ci(confidence, messages, losses).unwrap();
        println!("Loss rate less than: {}", ci);
    }
}
