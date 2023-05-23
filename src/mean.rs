//!
//! Confidence intervals over the mean (arithmetic, geometric, harmonic) of a given sample.
//!
//! The premise on which confidence intervals are computed is that the sample data is a random
//! sample from a population following some (unknown) distribution. The confidence interval
//! is computed from the sample data, and is an estimate of the true mean of the population.
//!
//! Unlike what is sometimes stated, the population does not need to be normally distributed.
//! However, it is assumed that the __standard error__ of the sample mean is normally distributed
//! (or close to it).
//! This is true for most distributions (especially symmetrical ones), and is guaranteed by
//! the central limit theorem as the size of the sample data grows large.
//!
//! The calculations use Student's t distribution almost regardless of sample size (until
//! a size of 100'000). This provides more conservative (and accurate intervals) than the
//! normal distribution when the number of samples is small, and asymptotically approaches
//! the normal distribution as the number of samples increases. This compensates for the
//! fact that the central limit theorem applies only asymptotically.
//!
//! # Assumptions
//!
//! The following assumptions are made:
//!
//! * The sample data is a random sample from a population following some (unknown) distribution.
//! * The sample data is independent and identically distributed (iid).
//! * The standard approaches a normal distribution.
//! * For geometric / harmonic means, the sample data is strictly positive.
//!
//! # Examples
//!
//! Confidence intervals on the arithmetic mean of a sample:
//! ```
//! use stats_ci::*;
//! let data = [
//!     82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
//!     15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
//!     71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
//!     98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
//!     49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
//!     37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
//! ];
//! let confidence = Confidence::new_two_sided(0.95);
//! let stats = mean::Arithmetic::from_iter(data)?;
//! // reference values computed in python / numpy
//! use approx::*;
//! assert_abs_diff_eq!(stats.sample_mean(), 53.67, epsilon = 1e-6);
//! assert_abs_diff_eq!(stats.sample_std_dev(), 28.097613040716798, epsilon = 1e-6);
//! assert_abs_diff_eq!(stats.ci_mean(confidence)?, Interval::new(48.094823990767836, 59.24517600923217)?, epsilon = 1e-6);
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! Confidence intervals on the geometric mean of a sample:
//! ```
//! # use stats_ci::*;
//! # let data = [
//! #    82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
//! #    15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
//! #    71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
//! #    98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
//! #    49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
//! #    37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
//! # ];
//! # let confidence = Confidence::new_two_sided(0.95);
//! let stats = mean::Geometric::from_iter(data)?;
//! // reference values computed in python / numpy
//! use approx::*;
//! assert_abs_diff_eq!(stats.sample_mean(), 43.7268032829256, epsilon = 1e-6);
//! assert_abs_diff_eq!(stats.ci_mean(confidence)?, Interval::new(37.731050052224354, 50.67532768627392)?, epsilon = 1e-6);
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! Confidence intervals on the harmonic mean of a sample:
//! ```
//! # use stats_ci::*;
//! # let data = [
//! #    82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
//! #    15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
//! #    71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
//! #    98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
//! #    49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
//! #    37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
//! # ];
//! # let confidence = Confidence::new_two_sided(0.95);
//! let stats = mean::Harmonic::from_iter(data)?;
//! // reference values computed in python / numpy
//! use approx::*;
//! assert_abs_diff_eq!(stats.sample_mean(), 30.031313156339586, epsilon = 1e-6);
//! assert_abs_diff_eq!(stats.ci_mean(confidence)?, Interval::new(23.614092539657168, 41.237860649168255)?, epsilon = 1e-6);
//! # Ok::<(),error::CIError>(())
//! ```
//!
use super::*;
use crate::utils;

use error::*;
use num_traits::Float;

///
/// Trait for incremental statistics.
/// This trait is implemented for the following statistics:
/// - [`mean::Arithmetic`] for arithmetic calculations
/// - [`mean::Geometric`] for geometric calculations (logarithmic space)
/// - [`mean::Harmonic`] for harmonic calculations (reciprocal space)
///
/// # Example
/// ```
/// use stats_ci::*;
/// let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
/// let stats = mean::Arithmetic::from_iter(data)?;
/// assert_eq!(stats.sample_count(), 10);
/// assert_eq!(stats.sample_mean(), 5.5);
/// assert_abs_diff_eq!(stats.sample_sem(), 1.0092, epsilon = 1e-4);
/// let confidence = Confidence::new_two_sided(0.95);
/// let ci = stats.ci_mean(confidence)?;
/// # use approx::*;
/// assert_abs_diff_eq!(ci, Interval::new(3.3341, 7.6659)?, epsilon = 1e-4);
/// # Ok::<(),error::CIError>(())
/// ```
pub trait StatisticsOps<F: Float>: Default {
    ///
    /// Create a new state and "populates" it with data from an iterator
    ///
    /// Complexity: \\( O(n) \\), where \\( n \\) is the number of elements in `data`
    ///
    /// # Arguments
    ///
    /// * `data` - The data to populate the state with
    ///
    /// # Errors
    ///
    /// * [`CIError::NonPositiveValue`] - If the input data contains non-positive values when computing harmonic/geometric means.
    ///
    /// # Example
    /// ```
    /// # use approx::*;
    /// use stats_ci::*;
    /// let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    /// let stats = mean::Arithmetic::from_iter(data)?;
    /// assert_eq!(stats.sample_count(), 10);
    /// assert_eq!(stats.sample_mean(), 5.5);
    /// assert_abs_diff_eq!(stats.sample_sem(), 1.0092, epsilon = 1e-4);
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    /// # Note
    ///
    /// This is simply a shortcut for [`Default::default`] and [`Self::extend`]:
    /// ```
    /// # use stats_ci::*;
    /// # let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    /// let stats = mean::Arithmetic::new().extend(data)?;
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    fn from_iter<I: IntoIterator<Item = F>>(data: I) -> CIResult<Self> {
        let mut state = Self::default();
        state.extend(data)?;
        Ok(state)
    }

    ///
    /// Mean of the sample
    ///
    /// Complexity: \\( O(1) \\)
    ///
    fn sample_mean(&self) -> F;

    ///
    /// Standard error of the sample mean
    ///
    /// Complexity: \\( O(1) \\)
    ///
    fn sample_sem(&self) -> F;

    ///
    /// Number of samples
    ///
    /// Complexity: \\( O(1) \\)
    ///
    fn sample_count(&self) -> usize;

    ///
    /// Confidence interval of the sample mean
    ///
    /// Complexity: \\( O(1) \\)
    ///
    fn ci_mean(&self, confidence: Confidence) -> CIResult<Interval<F>>;

    ///
    /// Append a new sample to the data
    ///
    /// Complexity: \\( O(1) \\)
    ///
    fn append(&mut self, x: F) -> CIResult<()>;

    ///
    /// Extend the data with additional sample data.
    ///
    /// This is equivalent to calling [`Self::append`] for each value in `data`.
    ///
    /// Complexity: \\( O(n) \\), where \\( n \\) is the number of elements in `data`
    ///
    /// # Arguments
    ///
    /// * `data` - The data to append as an array or an iterator
    ///
    /// # Output
    ///
    /// * `Ok(())` - If the data was successfully appended
    ///
    /// # Errors
    ///
    /// * [`CIError::NonPositiveValue`] - If the input data is invalid (for harmonic/geometric means).
    ///
    fn extend<I: IntoIterator<Item = F>>(&mut self, data: I) -> CIResult<()> {
        for x_i in data {
            self.append(x_i)?;
        }
        Ok(())
    }
}

///
/// Represents the state of the computation of the arithmetic mean.
/// This is a simple implementation that accumulates information about the samples, such as sum and sum of squares.
///
/// It is best used through the [`StatisticsOps`] trait.
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Arithmetic<F: Float> {
    sum: utils::KahanSum<F>,
    sum_sq: utils::KahanSum<F>,
    count: usize,
}

impl<F: Float> Default for Arithmetic<F> {
    fn default() -> Self {
        Self {
            sum: utils::KahanSum::default(),
            sum_sq: utils::KahanSum::default(),
            count: 0,
        }
    }
}

impl<F: Float> Arithmetic<F> {
    ///
    /// Create a new empty state
    ///
    /// # Example
    /// ```
    /// use stats_ci::*;
    /// let mut stats = mean::Arithmetic::new();
    /// stats.append(10.);
    /// assert_eq!(stats.sample_count(), 1);
    /// assert_eq!(stats.sample_mean(), 10.);
    /// ```
    ///
    pub fn new() -> Self {
        Default::default()
    }

    ///
    /// Variance of the sample
    /// \\( \frac{1}{n-1}\left(\sum_{i=1}^n x_i^2 - \frac{1}{n} \left(\sum_{i=1}^n x_i\right)^2 \right) \\)
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn sample_variance(&self) -> F {
        let mean = self.sample_mean();
        (self.sum_sq.value() - mean * self.sum.value()) / F::from(self.count - 1).unwrap()
    }

    ///
    /// Standard deviation of the sample
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn sample_std_dev(&self) -> F {
        self.sample_variance().sqrt()
    }
}

impl<F: Float> StatisticsOps<F> for Arithmetic<F> {
    fn append(&mut self, x: F) -> CIResult<()> {
        self.sum += x;
        self.sum_sq += x * x;
        self.count += 1;
        Ok(())
    }

    fn sample_mean(&self) -> F {
        self.sum.value() / F::from(self.count).unwrap()
    }

    fn sample_sem(&self) -> F {
        self.sample_std_dev() / F::from(self.count - 1).unwrap().sqrt()
    }

    fn ci_mean(&self, confidence: Confidence) -> CIResult<Interval<F>> {
        let n = self.count as f64;
        let mean = self.sample_mean().try_f64("stats.mean")?;
        let std_dev = self.sample_std_dev().try_f64("stats.std_dev")?;
        let std_err_mean = std_dev / n.sqrt();
        let degrees_of_freedom = n - 1.;
        let (lo, hi) = stats::interval_bounds(confidence, mean, std_err_mean, degrees_of_freedom);
        let (lo, hi) = (F::from(lo).convert("lo")?, F::from(hi).convert("hi")?);
        match confidence {
            Confidence::TwoSided(_) => Interval::new(lo, hi).map_err(|e| e.into()),
            Confidence::UpperOneSided(_) => Ok(Interval::new_upper(lo)),
            Confidence::LowerOneSided(_) => Ok(Interval::new_lower(hi)),
        }
    }

    fn sample_count(&self) -> usize {
        self.count
    }
}

impl<F: Float> std::ops::Add<Self> for Arithmetic<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.sum;
        let mut sum_sq = self.sum_sq;
        sum += rhs.sum;
        sum_sq += rhs.sum_sq;
        let count = self.count + rhs.count;
        Self { sum, sum_sq, count }
    }
}

///
/// Represents the state of the computation related to the harmonic mean.
/// This is a simple implementation that accumulates information about the samples, such as sum and sum of squares.
/// It is implemented as a wrapper around [`Arithmetic`] to compute the arithmetic mean of the reciprocals of the samples.
///
/// It is best used through the [`StatisticsOps`] trait.
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Harmonic<F: Float> {
    recip_space: Arithmetic<F>,
}

impl<F: Float> Harmonic<F> {
    ///
    /// Create a new empty state
    ///
    /// # Example
    /// ```
    /// use stats_ci::*;
    /// let mut stats = mean::Harmonic::new();
    /// stats.append(10.);
    /// assert_eq!(stats.sample_count(), 1);
    /// assert_eq!(stats.sample_mean(), 10.);
    /// ```
    ///
    pub fn new() -> Self {
        Default::default()
    }
}

impl<F: Float> Default for Harmonic<F> {
    fn default() -> Self {
        Self {
            recip_space: Arithmetic::default(),
        }
    }
}

impl<F: Float> StatisticsOps<F> for Harmonic<F> {
    fn append(&mut self, x: F) -> CIResult<()> {
        if x <= F::zero() {
            return Err(error::CIError::NonPositiveValue(
                x.to_f64().unwrap_or(f64::NAN),
            ));
        }
        self.recip_space.append(F::one() / x)?;
        Ok(())
    }

    ///
    /// Harmonic mean of the sample
    /// \\( H = \left( \frac{1}{n} \sum_i \frac{1}{x_i} \right)^{-1} \\)
    ///
    /// Complexity: \\( O(1) \\)
    ///
    fn sample_mean(&self) -> F {
        F::one() / self.recip_space.sample_mean()
    }

    ///
    /// Standard error of the harmonic mean
    /// \\( s_H = \frac{1}{\alpha^2} \frac{s_{1/x_i}}{\sqrt{n-1}} \\)
    ///
    /// where
    /// * the estimate of \\( \alpha \\) is given by \\( \alpha = \frac{1}{n} \sum_i 1/x_i \\);
    /// * \\( s_{1/x_i} \\) is the estimate of the standard deviation of the reciprocals of the samples;
    /// * and \\( n-1 \\) is the degree of freedom of the sample data.
    ///
    /// # Reference
    ///
    /// * Nilan Noris. "The standard errors of the geometric and harmonic means and their application to index numbers." Ann. Math. Statist. 11(4): 445-448 (December, 1940). DOI: [10.1214/aoms/1177731830](https://doi.org/10.1214/aoms/1177731830) [JSTOR](https://www.jstor.org/stable/2235727)
    ///
    fn sample_sem(&self) -> F {
        let harm_mean = self.sample_mean();
        let recip_std_dev = self.recip_space.sample_std_dev();
        harm_mean * harm_mean * recip_std_dev
            / F::from(self.recip_space.sample_count() - 1).unwrap().sqrt()
    }

    fn sample_count(&self) -> usize {
        self.recip_space.sample_count()
    }

    ///
    /// Confidence interval for the harmonic mean
    ///
    fn ci_mean(&self, confidence: Confidence) -> CIResult<Interval<F>> {
        let arith_ci = self.recip_space.ci_mean(confidence.flipped())?;
        let (lo, hi) = (F::one() / arith_ci.high_f(), F::one() / arith_ci.low_f());
        match confidence {
            Confidence::TwoSided(_) => Interval::new(lo, hi).map_err(|e| e.into()),
            Confidence::UpperOneSided(_) => Ok(Interval::new_upper(lo)),
            Confidence::LowerOneSided(_) => Ok(Interval::new_lower(hi)),
        }
    }
}

impl<F: Float> std::ops::Add<Self> for Harmonic<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            recip_space: self.recip_space + rhs.recip_space,
        }
    }
}

///
/// Represents the state of the computation of the geometric mean.
/// This is a simple implementation that accumulates information about the samples, such as sum and sum of squares.
/// It is implemented as a wrapper around [`Arithmetic`] to compute the arithmetic mean of the logarithms of the samples.
///
/// It is best used through the [`StatisticsOps`] trait.
///
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Geometric<F: Float> {
    log_space: Arithmetic<F>,
}

impl<F: Float> Geometric<F> {
    ///
    /// Create a new empty state
    ///
    /// # Example
    /// ```
    /// # use stats_ci::*;
    /// # use approx::*;
    /// let mut stats = mean::Geometric::new();
    /// stats.append(10.)?;
    /// assert_eq!(stats.sample_count(), 1);
    /// assert_abs_diff_eq!(stats.sample_mean(), 10., epsilon = 1e-10);
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    pub fn new() -> Self {
        Default::default()
    }
}

impl<F: Float> Default for Geometric<F> {
    fn default() -> Self {
        Self {
            log_space: Arithmetic::default(),
        }
    }
}

impl<F: Float> StatisticsOps<F> for Geometric<F> {
    fn append(&mut self, x: F) -> CIResult<()> {
        if x <= F::zero() {
            return Err(error::CIError::NonPositiveValue(
                x.to_f64().unwrap_or(f64::NAN),
            ));
        }
        self.log_space.append(x.ln())?;
        Ok(())
    }

    ///
    /// Geometric mean of the sample
    ///
    fn sample_mean(&self) -> F {
        self.log_space.sample_mean().exp()
    }

    ///
    /// Standard error of the geometric mean
    ///
    /// Computed as: \\( G \frac{s_{\log x_i}}{\sqrt{n-1}} \\)
    /// where \\( G \\) is the geometric mean of the sample;
    /// \\( s_{\log x_i} \\) is the estimate of the standard deviation of the logarithms of the samples;
    /// and \\( n-1 \\) is the degree of freedom of the sample data.
    ///
    ///  # Reference
    ///
    /// * Nilan Noris. "The standard errors of the geometric and harmonic means and their application to index numbers." Ann. Math. Statist. 11(4): 445-448 (December, 1940). DOI: [10.1214/aoms/1177731830](https://doi.org/10.1214/aoms/1177731830) [JSTOR](https://www.jstor.org/stable/2235727)
    ///
    fn sample_sem(&self) -> F {
        let geom_mean = self.sample_mean();
        let log_std_dev = self.log_space.sample_std_dev();
        geom_mean * log_std_dev / F::from(self.log_space.sample_count() - 1).unwrap().sqrt()
    }

    fn sample_count(&self) -> usize {
        self.log_space.sample_count()
    }

    ///
    /// Confidence interval for the geometric mean
    ///
    fn ci_mean(&self, confidence: Confidence) -> CIResult<Interval<F>> {
        let arith_ci = self.log_space.ci_mean(confidence)?;
        let (lo, hi) = (arith_ci.low_f().exp(), arith_ci.high_f().exp());
        match confidence {
            Confidence::TwoSided(_) => Interval::new(lo, hi).map_err(|e| e.into()),
            Confidence::UpperOneSided(_) => Ok(Interval::new_upper(lo)),
            Confidence::LowerOneSided(_) => Ok(Interval::new_lower(hi)),
        }
    }
}

impl<F: Float> std::ops::Add<Self> for Geometric<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            log_space: self.log_space + rhs.log_space,
        }
    }
}

///
/// Trait for computing confidence intervals on the mean of a sample.
///
/// It is superceded by the [`StatisticsOps`] trait which allows incremental statistics.
/// It is retained for backwards compatibility and will be deprecated in the future, as
/// it brings no advantage over [`StatisticsOps`] and is less flexible.
///
/// # Examples
///
/// ```
/// use stats_ci::*;
/// let data = [
///    82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
///    15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
///    71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
///    98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
///    49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
///    37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
/// ];
/// let confidence = Confidence::new_two_sided(0.95);
/// let ci = mean::Arithmetic::ci(confidence, data)?;
/// // arithmetic mean: 52.5
///
/// use num_traits::Float;
/// use approx::*;
/// assert_abs_diff_eq!(ci, Interval::new(48.094823990767836, 59.24517600923217)?, epsilon = 1e-3);
/// # Ok::<(),error::CIError>(())
/// ```
///
pub trait MeanCI<T: PartialOrd> {
    ///
    /// Compute the confidence interval on the mean of a sample
    ///
    /// # Arguments
    ///
    /// * `confidence` - The confidence level of the interval
    /// * `data` - The data to compute the confidence interval on
    ///
    /// # Output
    ///
    /// * `Ok(interval)` - The confidence interval on the mean of the sample
    ///
    /// # Errors
    ///
    /// * [`CIError::TooFewSamples`] - If the input data has too few samples to compute the confidence interval
    /// * [`CIError::NonPositiveValue`] - If the input data contains non-positive values when computing harmonic/geometric means.
    /// * [`CIError::InvalidInputData`] - If the input data contains invalid values (e.g. NaN)
    /// * [`CIError::FloatConversionError`] - If some data cannot be converted to a float
    ///
    fn ci<I>(confidence: Confidence, data: I) -> CIResult<Interval<T>>
    where
        I: IntoIterator<Item = T>;
}

impl<F: Float, T: StatisticsOps<F>> MeanCI<F> for T {
    fn ci<I>(confidence: Confidence, data: I) -> CIResult<Interval<F>>
    where
        I: IntoIterator<Item = F>,
    {
        Self::from_iter(data)?.ci_mean(confidence)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_mean_ci() -> CIResult<()> {
        let data = [
            82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
            15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
            71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
            98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
            49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
            37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
        ];

        let confidence = Confidence::new_two_sided(0.95);
        let ci = Arithmetic::ci(confidence, data)?;
        // mean: 53.67
        // stddev: 28.097613040716798
        // reference values computed in python
        // [48.094823990767836, 59.24517600923217]
        // ```python
        // import numpy as np
        // import scipy.stats as st
        // st.t.interval(confidence=0.95, df=len(data)-1, loc=np.mean(data), scale=st.sem(data))
        // ```
        assert_abs_diff_eq!(ci.low_f(), 48.094823990767836, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.high_f(), 59.24517600923217, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.low_f() + ci.high_f(), 2. * 53.67);

        let one_sided_ci = Arithmetic::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_abs_diff_eq!(one_sided_ci.low_f(), ci.low_f());
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = Arithmetic::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_abs_diff_eq!(one_sided_ci.high_f(), ci.high_f());
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        let mut state = Arithmetic::default();
        state.extend(data.iter().copied())?;
        let ci = state.ci_mean(confidence)?;

        assert_abs_diff_eq!(ci.low_f(), 48.094823990767836, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.high_f(), 59.24517600923217, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.low_f() + ci.high_f(), 2. * 53.67, epsilon = 1e-8);

        let one_sided_ci = state.ci_mean(Confidence::UpperOneSided(0.975))?;
        assert_abs_diff_eq!(one_sided_ci.low_f(), ci.low_f());
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = state.ci_mean(Confidence::LowerOneSided(0.975))?;
        assert_abs_diff_eq!(one_sided_ci.high_f(), ci.high_f());
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        Ok(())
    }

    #[test]
    fn test_geometric_ci() -> CIResult<()> {
        let data = [
            82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
            15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
            71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
            98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
            49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
            37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
        ];

        let confidence = Confidence::new_two_sided(0.95);
        let ci = Geometric::ci(confidence, data)?;
        // geometric mean: 43.7268032829256
        //
        // reference values computed in python:
        // in log space: (3.630483364286656, 3.9254391587458475)
        // [37.731050052224354, 50.67532768627392]
        assert_abs_diff_eq!(ci.low_f(), 37.731050052224354, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.high_f(), 50.67532768627392, epsilon = 1e-8);

        let one_sided_ci = Geometric::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_abs_diff_eq!(one_sided_ci.low_f(), ci.low_f());
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = Geometric::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_abs_diff_eq!(one_sided_ci.high_f(), ci.high_f());
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        let mut state = Geometric::default();
        state.extend(data.iter().copied())?;
        let ci = state.ci_mean(confidence)?;
        assert_abs_diff_eq!(state.sample_mean(), 43.7268032829256, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.low_f(), 37.731050052224354, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.high_f(), 50.67532768627392, epsilon = 1e-8);

        let one_sided_ci = state.ci_mean(Confidence::UpperOneSided(0.975))?;
        assert_abs_diff_eq!(one_sided_ci.low_f(), ci.low_f());
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = state.ci_mean(Confidence::LowerOneSided(0.975))?;
        assert_abs_diff_eq!(one_sided_ci.high_f(), ci.high_f());
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        Ok(())
    }

    #[test]
    fn test_harmonic_ci() -> CIResult<()> {
        let data = [
            82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
            15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
            71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
            98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
            49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
            37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
        ];

        let confidence = Confidence::new_two_sided(0.95);
        let ci = Harmonic::ci(confidence, data)?;
        // harmonic mean: 30.031313156339586
        //
        // reference values computed in python:
        // in reciprocal space: (0.02424956057996111, 0.042347593849757906)
        // [41.237860649168255, 23.614092539657168]  (reversed by conversion from reciprocal space)
        assert_abs_diff_eq!(ci.low_f(), 23.614092539657168, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.high_f(), 41.237860649168255, epsilon = 1e-8);

        let one_sided_ci = Harmonic::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_abs_diff_eq!(one_sided_ci.low_f(), ci.low_f());
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = Harmonic::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_abs_diff_eq!(one_sided_ci.high_f(), ci.high_f());
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        let confidence = Confidence::new_two_sided(0.95);
        let data = [
            1.81600583, 0.07498389, 1.29092744, 0.62023863, 0.09345327, 1.94670997, 2.27687339,
            0.9251231, 1.78173864, 0.4391542, 1.36948099, 1.5191194, 0.42286756, 1.48463176,
            0.17621009, 2.31810064, 0.15633061, 2.55137878, 1.11043948, 1.35923319, 1.58385561,
            0.63431437, 0.49993148, 0.49168534, 0.11533354,
        ];
        let ci = Harmonic::ci(confidence, data)?;
        // harmonic mean: 0.38041820166550844
        //
        // reference values computed in python:
        // in reciprocal space: (1.1735238063066096, 4.083848080632111)
        // [0.8521343961033607, 0.2448670911003175]
        assert_abs_diff_eq!(ci.low_f(), 0.2448670911003175, epsilon = 1e-6);
        assert_abs_diff_eq!(ci.high_f(), 0.8521343961033607, epsilon = 1e-6);

        let mut state = Harmonic::default();
        state.extend(data.iter().copied())?;
        let ci = state.ci_mean(confidence)?;
        assert_abs_diff_eq!(state.sample_mean(), 0.38041820166550844, epsilon = 1e-8);
        assert_abs_diff_eq!(ci.low_f(), 0.2448670911003175, epsilon = 1e-6);
        assert_abs_diff_eq!(ci.high_f(), 0.8521343961033607, epsilon = 1e-6);

        Ok(())
    }

    #[test]
    fn test_stats_sum() {
        const VALUE: f32 = 0.1;
        let size = 1_000_000;

        let mut stats_ref = Arithmetic::default();
        let mut stats_summed = Arithmetic::default();
        for _ in 0..size {
            stats_ref.append(VALUE).unwrap();

            let mut new_stat = Arithmetic::default();
            new_stat.append(VALUE).unwrap();
            stats_summed = stats_summed + new_stat;
        }

        assert_eq!(stats_ref.sample_count(), size);
        assert_eq!(stats_summed.sample_count(), size);

        assert_eq!(stats_ref.sample_mean(), stats_summed.sample_mean());
        assert_eq!(stats_ref.sample_variance(), stats_summed.sample_variance());
        assert_eq!(stats_ref.sample_std_dev(), stats_summed.sample_std_dev());
        assert_eq!(stats_ref.sample_sem(), stats_summed.sample_sem());
    }

    #[test]
    fn test_blah() -> CIResult<()> {
        let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
        let stats = mean::Arithmetic::from_iter(data)?;
        assert_eq!(stats.sample_count(), 10);
        assert_eq!(stats.sample_mean(), 5.5);
        assert_abs_diff_eq!(stats.sample_sem(), 1.0092, epsilon = 1e-4);
        let confidence = Confidence::new_two_sided(0.95);
        let ci = stats.ci_mean(confidence)?;
        assert_abs_diff_eq!(ci, Interval::new(3.3341, 7.6659)?, epsilon = 1e-4);
        Ok(())
    }
}
