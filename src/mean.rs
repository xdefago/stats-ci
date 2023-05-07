//! Confidence intervals over the mean (arithmetic, geometric, harmonic) of a given sample.
//!
//! The calculations use Student's t distribution regardless of sample size.
//! This provides more conservative (and accurate intervals) than the normal distribution
//! when the number of samples is small, and asymptotically approaches the normal distribution
//! as the number of samples increases.
//!
//! # Examples
//!
//! Confidence intervals on the arithmetic mean of a sample:
//! ```
//! # fn test() -> stats_ci::CIResult<()> {
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
//! let ci = mean::Arithmetic::ci(confidence, data)?;
//! // mean: 53.67
//! // stddev: 28.097613040716798
//! // reference values computed in python
//! // [48.094823990767836, 59.24517600923217]
//! use num_traits::Float;
//! use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low_f(), 48.094823990767836, 1e-6);
//! assert_approx_eq!(ci.high_f(), 59.24517600923217, 1e-6);
//! # Ok(())
//! # }
//! ```
//!
//! Confidence intervals on the geometric mean of a sample:
//! ```
//! # fn test() -> stats_ci::CIResult<()> {
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
//! let ci = mean::Geometric::ci(confidence, data)?;
//! // geometric mean: 43.7268032829256
//! // reference values computed in python:
//! // [37.731050052224354, 50.67532768627392]
//! # use num_traits::Float;
//! # use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low_f(), 37.731050052224354, 1e-6);
//! assert_approx_eq!(ci.high_f(), 50.67532768627392, 1e-6);
//! # Ok(())
//! # }
//! ```
//!
//! Confidence intervals on the harmonic mean of a sample:
//! ```
//! # fn test() -> stats_ci::CIResult<()> {
//! # use stats_ci::*;
//! # let data = [
//! #     1.81600583, 0.07498389, 1.29092744, 0.62023863, 0.09345327, 1.94670997, 2.27687339,
//! #     0.9251231, 1.78173864, 0.4391542, 1.36948099, 1.5191194, 0.42286756, 1.48463176,
//! #     0.17621009, 2.31810064, 0.15633061, 2.55137878, 1.11043948, 1.35923319, 1.58385561,
//! #     0.63431437, 0.49993148, 0.49168534, 0.11533354,
//! # ];
//! # let confidence = Confidence::new_two_sided(0.95);
//! let ci = mean::Harmonic::ci(confidence, data.clone())?;
//! // harmonic mean: 0.38041820166550844
//! // reference values computed in python:
//! // [0.2448670911003175, 0.8521343961033607]
//! # use num_traits::Float;
//! # use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low_f(), 0.2448670911003175, 1e-6);
//! assert_approx_eq!(ci.high_f(), 0.8521343961033607, 1e-6);
//! # Ok(())
//! # }
//! ```
//!
//! The confidence interval and relevant statistics can also be computed incrementally:
//! ```
//! # fn test() -> stats_ci::CIResult<()> {
//! # use stats_ci::*;
//! # let data = [
//! #    55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23., 37., 72., 62., 77.,
//! #    63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
//! # ];
//! let stats = mean::Arithmetic::from_iter(data)?;
//! let confidence = Confidence::new_two_sided(0.95);
//! let ci = stats.ci_mean(confidence)?;
//! # Ok(())
//! # }
//! ```
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
/// # fn test() -> stats_ci::CIResult<()> {
/// use stats_ci::*;
/// let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
/// let stats = mean::Arithmetic::from_iter(data)?;
/// assert_eq!(stats.sample_count(), 10);
/// assert_eq!(stats.sample_mean(), 5.5);
/// assert_eq!(stats.sample_sem(), 0.5);
/// let confidence = Confidence::new_two_sided(0.95);
/// let ci = stats.ci_mean(confidence)?;
/// assert_eq!(ci.low_f(), 3.5420208206306123);
/// assert_eq!(ci.high_f(), 7.457979179369388);
/// # Ok(())
/// # }
/// ```
pub trait StatisticsOps<F: Float>
where
    Self: Default,
{
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
    fn new() -> Self {
        Default::default()
    }
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
    /// # fn test() -> stats_ci::CIResult<()> {
    /// use stats_ci::*;
    /// let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    /// let stats = mean::Arithmetic::from_iter(data)?;
    /// assert_eq!(stats.sample_count(), 10);
    /// assert_eq!(stats.sample_mean(), 5.5);
    /// assert_eq!(stats.sample_sem(), 0.5);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Note
    ///
    /// This is simply a shortcut for [`Self::new`] and [`Self::extend`]:
    /// ```
    /// # fn test() -> stats_ci::CIResult<()> {
    /// # use stats_ci::*;
    /// # let data = [1., 2., 3., 4., 5., 6., 7., 8., 9., 10.];
    /// let stats = mean::Arithmetic::new().extend(data)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    fn from_iter<I: IntoIterator<Item = F>>(data: I) -> CIResult<Self> {
        let mut state = Self::new();
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
    sum: F,
    sum_c: F,
    sum_sq: F,
    sum_sq_c: F,
    count: usize,
}

impl<F: Float> Default for Arithmetic<F> {
    fn default() -> Self {
        Self {
            sum: F::zero(),
            sum_c: F::zero(),
            sum_sq: F::zero(),
            sum_sq_c: F::zero(),
            count: 0,
        }
    }
}

impl<F: Float> Arithmetic<F> {
    ///
    /// Variance of the sample
    /// \\( \frac{1}{n-1}\left(\sum_{i=1}^n x_i^2 - \frac{1}{n} \left(\sum_{i=1}^n x_i\right)^2 \right) \\)
    ///
    /// Complexity: \\( O(1) \\)
    ///
    pub fn sample_variance(&self) -> F {
        let mean = self.sample_mean();
        (self.sum_sq - mean * self.sum) / F::from(self.count - 1).unwrap()
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
        utils::kahan_add(&mut self.sum, x, &mut self.sum_c);
        utils::kahan_add(&mut self.sum_sq, x * x, &mut self.sum_sq_c);
        self.count += 1;
        Ok(())
    }

    fn sample_mean(&self) -> F {
        self.sum / F::from(self.count).unwrap()
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
        let mut sum_c = self.sum_c;
        utils::kahan_add(&mut sum, rhs.sum_c, &mut sum_c);
        utils::kahan_add(&mut sum, rhs.sum, &mut sum_c);
        let mut sum_sq = self.sum_sq;
        let mut sum_sq_c = self.sum_sq_c;
        utils::kahan_add(&mut sum_sq, rhs.sum_sq_c, &mut sum_sq_c);
        utils::kahan_add(&mut sum_sq, rhs.sum_sq, &mut sum_sq_c);
        let count = self.count + rhs.count;
        Self {
            sum,
            sum_c,
            sum_sq,
            sum_sq_c,
            count,
        }
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
/// # fn test() -> stats_ci::CIResult<()> {
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
/// use assert_approx_eq::assert_approx_eq;
/// assert_approx_eq!(ci.low_f(), 41.6496, 1e-3);
/// assert_approx_eq!(ci.high_f(), 65.69, 1e-3);
/// # Ok(())
/// # }
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
    use assert_approx_eq::assert_approx_eq;

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
        assert_approx_eq!(ci.low_f(), 48.094823990767836, 1e-8);
        assert_approx_eq!(ci.high_f(), 59.24517600923217, 1e-8);
        assert_approx_eq!(ci.low_f() + ci.high_f(), 2. * 53.67, 1e-8);

        let one_sided_ci = Arithmetic::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_approx_eq!(one_sided_ci.low_f(), ci.low_f(), 1e-8);
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = Arithmetic::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_approx_eq!(one_sided_ci.high_f(), ci.high_f(), 1e-8);
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        let mut state = Arithmetic::default();
        state.extend(data.iter().copied())?;
        let ci = state.ci_mean(confidence)?;

        assert_approx_eq!(ci.low_f(), 48.094823990767836, 1e-8);
        assert_approx_eq!(ci.high_f(), 59.24517600923217, 1e-8);
        assert_approx_eq!(ci.low_f() + ci.high_f(), 2. * 53.67, 1e-8);

        let one_sided_ci = state.ci_mean(Confidence::UpperOneSided(0.975))?;
        assert_approx_eq!(one_sided_ci.low_f(), ci.low_f(), 1e-8);
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = state.ci_mean(Confidence::LowerOneSided(0.975))?;
        assert_approx_eq!(one_sided_ci.high_f(), ci.high_f(), 1e-8);
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
        assert_approx_eq!(ci.low_f(), 37.731050052224354, 1e-8);
        assert_approx_eq!(ci.high_f(), 50.67532768627392, 1e-8);

        let one_sided_ci = Geometric::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_approx_eq!(one_sided_ci.low_f(), ci.low_f(), 1e-8);
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = Geometric::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_approx_eq!(one_sided_ci.high_f(), ci.high_f(), 1e-8);
        assert_eq!(one_sided_ci.low_f(), f64::NEG_INFINITY);

        let mut state = Geometric::default();
        state.extend(data.iter().copied())?;
        let ci = state.ci_mean(confidence)?;
        assert_approx_eq!(state.sample_mean(), 43.7268032829256, 1e-8);
        assert_approx_eq!(ci.low_f(), 37.731050052224354, 1e-8);
        assert_approx_eq!(ci.high_f(), 50.67532768627392, 1e-8);

        let one_sided_ci = state.ci_mean(Confidence::UpperOneSided(0.975))?;
        assert_approx_eq!(one_sided_ci.low_f(), ci.low_f(), 1e-8);
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = state.ci_mean(Confidence::LowerOneSided(0.975))?;
        assert_approx_eq!(one_sided_ci.high_f(), ci.high_f(), 1e-8);
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
        // [41.237860649168255, 23.614092539657168]  (reversed by conversion to reciprocal space)
        assert_approx_eq!(ci.low_f(), 23.614092539657168, 1e-8);
        assert_approx_eq!(ci.high_f(), 41.237860649168255, 1e-8);

        let one_sided_ci = Harmonic::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_approx_eq!(one_sided_ci.low_f(), ci.low_f(), 1e-8);
        assert_eq!(one_sided_ci.high_f(), f64::INFINITY);

        let one_sided_ci = Harmonic::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_approx_eq!(one_sided_ci.high_f(), ci.high_f(), 1e-8);
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
        assert_approx_eq!(ci.low_f(), 0.2448670911003175, 1e-6);
        assert_approx_eq!(ci.high_f(), 0.8521343961033607, 1e-6);

        let mut state = Harmonic::default();
        state.extend(data.iter().copied())?;
        let ci = state.ci_mean(confidence)?;
        assert_approx_eq!(state.sample_mean(), 0.38041820166550844, 1e-8);
        assert_approx_eq!(ci.low_f(), 0.2448670911003175, 1e-6);
        assert_approx_eq!(ci.high_f(), 0.8521343961033607, 1e-6);

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
}
