//!
//!
//! Comparison of two samples
//! =========================
//!
//! This module provides functions to compare two samples for two different cases.
//!
//! # Paired observations
//!
//! The first case is when the two samples are paired, i.e. each measurement in the first sample is paired with a measurement in the second sample.
//! For instance, when measuring the performance of two algorithms, the same input data is used for both algorithms to yield a pair of related observations.
//! Obviously, the number of observations in the two samples must be the same.
//! When possible, paired observations are preferred because they significantly reduce the variance of the difference between the two means.
//! This means that fewer observations are needed to achieve the same significance.
//!
//! The function [`paired_ci`] computes the confidence interval of the difference between the means of the two samples.
//!
//! # Unpaired observations
//!
//! The second case is when the two samples are not paired, i.e. each measurement in the first sample is not paired with a measurement in the second sample.
//! The number of observations in the two samples may be different.
//!
//! The function [`unpaired_ci`] computes the confidence interval of the difference between the means of the two samples.
//!
//! # Examples
//!
//! ## Paired observations
//! ```
//! # use stats_ci::*;
//! // Zinc concentration in water samples from a river
//! let data_bottom_water = [
//!    0.430, 0.266, 0.567, 0.531, 0.707, 0.716, 0.651, 0.589, 0.469, 0.723,
//! ];
//! let data_surface_water = [
//!   0.415, 0.238, 0.390, 0.410, 0.605, 0.609, 0.632, 0.523, 0.411, 0.612,
//! ];
//! let mut stats = comparison::Paired::default();
//! stats.extend(data_bottom_water, data_surface_water)?;
//! let ci = stats.ci_mean(Confidence::new_two_sided(0.95))?;
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! ## Unpaired observations
//! ```
//! # use stats_ci::*;
//! // Gain in weight of 19 female rats between 28 and 84 days after birth.
//! // 12 were fed on a high protein diet and 7 on a low protein diet.
//! let data_high_protein = [
//!     134., 146., 104., 119., 124., 161., 107., 83., 113., 129., 97., 123.,
//! ];
//! let data_low_protein = [70., 118., 101., 85., 107., 132., 94.];
//! let mut stats = comparison::Unpaired::default();
//! stats.extend(data_high_protein, data_low_protein)?;
//! let ci = stats.ci_mean(Confidence::new_two_sided(0.95))?;
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! # References
//!
//! * R. Jain, The Art of Computer Systems Performance Analysis, Wiley, 1991.
//! * [Wikipedia article on paired difference test](https://en.wikipedia.org/wiki/Paired_difference_test)
//! * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
//!
use crate::*;
use error::*;
use mean::StatisticsOps;
use num_traits::Float;

///
/// Structure to collect statistics on two paired samples.
///
/// # Examples
///
/// ```
/// # use stats_ci::*;
/// // based on an example from https://online.stat.psu.edu/stat500/lesson/7/7.3/7.3.2
/// // Zinc concentration in water samples from a river
/// let data_bottom_water = [
///     0.430, 0.266, 0.567, 0.531, 0.707, 0.716, 0.651, 0.589, 0.469, 0.723,
/// ];
/// let data_surface_water = [
///     0.415, 0.238, 0.390, 0.410, 0.605, 0.609, 0.632, 0.523, 0.411, 0.612,
/// ];
///
/// let mut stats = comparison::Paired::default();
/// stats.extend(data_bottom_water, data_surface_water).unwrap();
/// let ci = stats.ci_mean(Confidence::new_two_sided(0.95)).unwrap();
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Paired<T: Float> {
    stats: mean::Arithmetic<T>,
}

impl<T: Float> Paired<T> {
    pub fn append_pair(&mut self, data_a: T, data_b: T) -> CIResult<()> {
        self.append(data_a - data_b)
    }

    ///
    /// Append pairs of observations to the two samples.
    ///
    /// # Arguments
    ///
    /// * `iter` - an iterable collection of tuples to add to the data
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let mut stats = comparison::Paired::default();
    /// stats.extend_tuple([(1., 2.), (3., 4.)])?;
    /// # assert_eq!(stats.sample_count(), 2);
    /// # assert_eq!(stats.sample_mean(), -1.);
    /// # Ok::<(),error::CIError>(())
    /// ```
    pub fn extend_tuple<I>(&mut self, iter: I) -> CIResult<()>
    where
        I: IntoIterator<Item = (T, T)>,
    {
        self.stats.extend(iter.into_iter().map(|(x, y)| x - y))
    }

    pub fn extend<I1, I2>(&mut self, data_a: I1, data_b: I2) -> CIResult<()>
    where
        I1: IntoIterator<Item = T>,
        I2: IntoIterator<Item = T>,
    {
        let mut data_a = data_a.into_iter();
        let mut data_b = data_b.into_iter();
        let mut count = 0;
        loop {
            match (data_a.next(), data_b.next()) {
                (Some(x), Some(y)) => {
                    count += 1;
                    self.append(x - y)?
                }
                (None, None) => return Ok(()),
                // returns error if iterables have different lengths
                (None, _) => {
                    return Err(CIError::DifferentSampleSizes(
                        count,
                        count + 1 + data_b.count(),
                    ))
                }
                (_, None) => {
                    return Err(CIError::DifferentSampleSizes(
                        count + 1 + data_a.count(),
                        count,
                    ))
                }
            }
        }
    }

    ///
    /// Compute the confidence interval of the difference between the means of the two samples.
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level
    /// * `data_a` - the first sample
    /// * `data_b` - the second sample
    ///
    /// # Returns
    ///
    /// The confidence interval of the difference as a result.
    ///
    /// # Errors
    ///
    /// * [`CIError::DifferentSampleSizes`] - if the two samples do not have the same length
    ///
    /// # Notes
    ///
    /// If the interval includes zero, the difference is not significant.
    /// If the interval is strictly positive (resp. negative), the mean of the first sample is significantly
    /// greater (resp. smaller) than the mean of the second sample.
    ///
    /// # References
    ///
    /// * R. Jain, The Art of Computer Systems Performance Analysis, Wiley, 1991.
    /// * [Wikipedia article on paired difference test](https://en.wikipedia.org/wiki/Paired_difference_test)
    /// * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let data_a = [1., 2., 3.];
    /// let data_b = [4., 5., 6.];
    /// let confidence = Confidence::new_two_sided(0.95);
    /// let ci = comparison::Paired::ci(confidence, &data_a, &data_b)?;
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    pub fn ci(confidence: Confidence, data_a: &[T], data_b: &[T]) -> CIResult<Interval<T>> {
        let mut stats = Paired::default();
        stats.extend(data_a.into_iter().copied(), data_b.into_iter().copied())?;
        stats.ci_mean(confidence)
    }
}

impl<T: Float> Default for Paired<T> {
    fn default() -> Self {
        Self {
            stats: mean::Arithmetic::default(),
        }
    }
}

impl<T: Float> mean::StatisticsOps<T> for Paired<T> {
    fn sample_mean(&self) -> T {
        self.stats.sample_mean()
    }

    fn sample_sem(&self) -> T {
        self.stats.sample_sem()
    }

    fn sample_count(&self) -> usize {
        self.stats.sample_count()
    }

    fn ci_mean(&self, confidence: Confidence) -> CIResult<Interval<T>> {
        self.stats.ci_mean(confidence)
    }

    fn append(&mut self, x: T) -> CIResult<()> {
        self.stats.append(x)
    }
}

impl<F: Float> std::ops::Add<Self> for Paired<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            stats: self.stats + rhs.stats,
        }
    }
}

///
/// Given two samples such that each measurement in the first sample is paired with a measurement in
/// the second sample, compute the confidence interval of the difference between the means of the
/// two samples.
///
/// # Arguments
///
/// * `confidence` - the confidence level
/// * `data1` - the first sample
/// * `data2` - the second sample
///
/// # Returns
///
/// The confidence interval of the difference as a result.
///
/// # Errors
///
/// * [`CIError::DifferentSampleSizes`] - if the two samples do not have the same length
///
/// # Notes
///
/// If the interval includes zero, the difference is not significant.
/// If the interval is strictly positive (resp. negative), the mean of the first sample is significantly
/// greater (resp. smaller) than the mean of the second sample.
///
/// # References
///
/// * R. Jain, The Art of Computer Systems Performance Analysis, Wiley, 1991.
/// * [Wikipedia article on paired difference test](https://en.wikipedia.org/wiki/Paired_difference_test)
/// * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
///
pub fn paired_ci<T: Float>(
    confidence: Confidence,
    data_a: &[T],
    data_b: &[T],
) -> CIResult<Interval<T>> {
    let mut stats = Paired::default();
    stats.extend(data_a.into_iter().copied(), data_b.into_iter().copied())?;
    stats.ci_mean(confidence)
}

///
/// Structure to collect statistics on two unpaired samples.
///
/// # Examples
/// 
/// ```
/// # use stats_ci::*;
/// // Gain in weight of 19 female rats between 28 and 84 days after birth.
/// // 12 were fed on a high protein diet and 7 on a low protein diet.
/// let data_high_protein = [
///     134., 146., 104., 119., 124., 161., 107., 83., 113., 129., 97., 123.,
/// ];
/// let data_low_protein = [70., 118., 101., 85., 107., 132., 94.];
/// let mut stats = comparison::Unpaired::default();
/// stats.extend(data_high_protein, data_low_protein)?;
/// let ci = stats.ci_mean(Confidence::new_two_sided(0.95))?;
/// # Ok::<(),error::CIError>(())
/// ```
/// 
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unpaired<T: Float> {
    stats_a: mean::Arithmetic<T>,
    stats_b: mean::Arithmetic<T>,
}

impl<T: Float> Default for Unpaired<T> {
    fn default() -> Self {
        Self {
            stats_a: mean::Arithmetic::default(),
            stats_b: mean::Arithmetic::default(),
        }
    }
}

impl<T: Float> Unpaired<T> {
    ///
    /// Create a new instance of `Unpaired` from two statistics.
    ///
    /// # Arguments
    ///
    /// * `stats_a` - the statistics of the first sample
    /// * `stats_b` - the statistics of the second sample
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let stats_a = mean::Arithmetic::from_iter([1., 2., 3.])?;
    /// let stats_b = mean::Arithmetic::from_iter([4., 5., 6.])?;
    /// let stats = comparison::Unpaired::new(stats_a, stats_b);
    /// # Ok::<(),error::CIError>(())
    /// ```
    pub fn new(stats_a: mean::Arithmetic<T>, stats_b: mean::Arithmetic<T>) -> Self {
        Self { stats_a, stats_b }
    }

    ///
    /// Create a new instance of `Unpaired` from two samples.
    ///
    /// # Arguments
    ///
    /// * `data_a` - the first sample
    /// * `data_b` - the second sample
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let stats = comparison::Unpaired::from_iter([1., 2., 3.], [4., 5., 6.])?;
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    pub fn from_iter<Ia, Ib>(data_a: Ia, data_b: Ib) -> CIResult<Self>
    where
        Ia: IntoIterator<Item = T>,
        Ib: IntoIterator<Item = T>,
    {
        let mut state = Self::default();
        state.extend_a(data_a)?;
        state.extend_b(data_b)?;
        Ok(state)
    }

    ///
    /// Return a reference to the statistics of the first sample.
    ///
    pub fn stats_a(&self) -> &mean::Arithmetic<T> {
        &self.stats_a
    }

    ///
    /// Return a mutable reference to the statistics of the first sample.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let mut stats = comparison::Unpaired::default();
    /// stats.stats_a_mut().append(1.)?;
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    pub fn stats_a_mut(&mut self) -> &mut mean::Arithmetic<T> {
        &mut self.stats_a
    }

    ///
    /// Return a reference to the statistics of the second sample.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// # let mut stats = comparison::Unpaired::from_iter([1., 2. ,3.], [4., 5., 6.])?;
    /// let mean_b = stats.stats_b().sample_mean();
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    pub fn stats_b(&self) -> &mean::Arithmetic<T> {
        &self.stats_b
    }

    ///
    /// Return a mutable reference to the statistics of the second sample.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let mut stats = comparison::Unpaired::default();
    /// stats.stats_b_mut().append(1.)?;
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    pub fn stats_b_mut(&mut self) -> &mut mean::Arithmetic<T> {
        &mut self.stats_b
    }

    ///
    /// Append a pair of observations to the two samples.
    ///
    /// # Arguments
    ///
    /// * `data_a` - the new data for the first sample
    /// * `data_b` - the new data for the second sample
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    ///
    pub fn append_pair(&mut self, data_a: T, data_b: T) -> CIResult<()> {
        self.append_a(data_a)?;
        self.append_b(data_b)?;
        Ok(())
    }

    ///
    /// Append a single observation to the first sample.
    ///
    /// # Arguments
    ///
    /// * `data_a` - the new data for the first sample
    ///
    pub fn append_a(&mut self, data_a: T) -> CIResult<()> {
        self.stats_a.append(data_a)
    }

    ///
    /// Append a single observation to the second sample.
    ///
    /// # Arguments
    ///
    /// * `data_b` - the new data for the second sample
    ///
    pub fn append_b(&mut self, data_b: T) -> CIResult<()> {
        self.stats_b.append(data_b)
    }

    ///
    /// Append observations to the first sample.
    ///
    /// # Arguments
    ///
    /// * `data_a` - the new data for the first sample
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let mut stats = comparison::Unpaired::default();
    /// stats.extend_a([1., 2., 3.])?;
    /// # assert_eq!(stats.stats_a().sample_count(), 3);
    /// # assert_eq!(stats.stats_a().sample_mean(), 2.);
    /// # Ok::<(),error::CIError>(())
    /// ```
    pub fn extend_a(&mut self, data_a: impl IntoIterator<Item = T>) -> CIResult<()> {
        self.stats_a.extend(data_a)
    }

    ///
    /// Append observations to the second sample.
    ///
    /// # Arguments
    ///
    /// * `data_b` - the new data for the second sample
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let mut stats = comparison::Unpaired::default();
    /// stats.extend_b([1., 2., 3.])?;
    /// # assert_eq!(stats.stats_b().sample_count(), 3);
    /// # assert_eq!(stats.stats_b().sample_mean(), 2.);
    /// # Ok::<(),error::CIError>(())
    /// ```
    pub fn extend_b(&mut self, data_b: impl IntoIterator<Item = T>) -> CIResult<()> {
        self.stats_b.extend(data_b)
    }

    ///
    /// Extend the two samples with new data.
    ///
    /// # Arguments
    ///
    /// * `data_a` - the new data for the first sample
    /// * `data_b` - the new data for the second sample
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let mut stats = comparison::Unpaired::default();
    /// stats.extend([1., 2., 3.], [4., 5., 6.])?;
    /// # assert_eq!(stats.stats_a().sample_count(), 3);
    /// # assert_eq!(stats.stats_a().sample_mean(), 2.);
    /// # assert_eq!(stats.stats_b().sample_count(), 3);
    /// # assert_eq!(stats.stats_b().sample_mean(), 5.);
    /// # Ok::<(),error::CIError>(())
    /// ```
    pub fn extend<Ia, Ib>(&mut self, data_a: Ia, data_b: Ib) -> CIResult<()>
    where
        Ia: IntoIterator<Item = T>,
        Ib: IntoIterator<Item = T>,
    {
        self.stats_a.extend(data_a)?;
        self.stats_b.extend(data_b)?;
        Ok(())
    }

    ///
    /// Compute the confidence interval of the difference between the means of the two samples.
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level
    ///
    /// # Returns
    ///
    /// The confidence interval of the difference as a result.
    ///
    /// # Errors
    ///
    /// * [`CIError::TooFewSamples`] - if one of the two samples has less than 2 observations
    ///
    /// # Notes
    ///
    /// If the interval includes zero, the difference is not significant.
    /// If the interval is strictly positive (resp. negative), the mean of the first sample is significantly
    /// greater (resp. smaller) than the mean of the second sample.
    ///
    /// # References
    ///
    /// * R. Jain, The Art of Computer Systems Performance Analysis, Wiley, 1991.
    /// * [Wikipedia article on Student's t-test](https://en.wikipedia.org/wiki/Student%27s_t-test#Independent_two-sample_t-test)
    /// * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
    ///
    pub fn ci_mean(&self, confidence: Confidence) -> CIResult<Interval<T>> {
        let stats_a = self.stats_a;
        let stats_b = self.stats_b;

        let n_a = T::from(stats_a.sample_count()).convert("stats_a.sample_count")?;
        let n_b = T::from(stats_b.sample_count()).convert("stats_b.sample_count")?;
        let mean_a = stats_a.sample_mean();
        let mean_b = stats_b.sample_mean();
        let std_dev_a = stats_a.sample_std_dev();
        let std_dev_b = stats_b.sample_std_dev();

        let mean_difference = mean_a - mean_b;
        let sa2_na = // $s_a^2 / n_a$
            std_dev_a * std_dev_a / n_a;
        let sb2_nb = // $s_b^2 / n_b$
            std_dev_b * std_dev_b / n_b;
        let sum_s2_n = // $s_a^2 / n_a + s_b^2 / n_b$
            sa2_na + sb2_nb;
        let std_err_mean = // $\sqrt{s_a^2 / n_a + s_b^2 / n_b}$
            sum_s2_n.sqrt();
        let effective_dof = // $ \frac{ (s_a^a / n_a + s_b^2 / n_b)^2 }{ \frac{1}{n_a+1} \left(\frac{s_a^2}{n_a}\right)^2 + \frac{1}{n_b+1} \left(\frac{s_b^2}{n_b}\right)^2 } - 2$
            sum_s2_n * sum_s2_n
                / (sa2_na * sa2_na / (n_a + T::one())
                    + sb2_nb * sb2_nb / (n_b + T::one())) - T::one() - T::one();

        let (lo, hi) = stats::interval_bounds(
            confidence,
            mean_difference.try_f64("mean_difference")?,
            std_err_mean.try_f64("std_err_mean")?,
            effective_dof.try_f64("effective_dof")?,
        );
        let lo = T::from(lo).convert("lo")?;
        let hi = T::from(hi).convert("hi")?;
        match confidence {
            Confidence::TwoSided(_) => Interval::new(lo, hi).map_err(|e| e.into()),
            Confidence::UpperOneSided(_) => Ok(Interval::new_upper(lo)),
            Confidence::LowerOneSided(_) => Ok(Interval::new_lower(hi)),
        }
    }

    ///
    /// Compute the confidence interval of the difference between the means of the two samples.
    ///
    /// # Arguments
    ///
    /// * `confidence` - the confidence level
    /// * `data_a` - the first sample
    /// * `data_b` - the second sample
    ///
    /// # Returns
    ///
    /// The confidence interval of the difference as a result.
    ///
    /// # Errors
    ///
    /// * [`CIError::FloatConversionError`] - if the conversion to `T` fails
    /// * [`CIError::TooFewSamples`] - if one of the two samples has less than 2 observations
    ///
    /// # Notes
    ///
    /// If the interval includes zero, the difference is not significant.
    /// If the interval is strictly positive (resp. negative), the mean of the first sample is significantly
    /// greater (resp. smaller) than the mean of the second sample.
    ///
    /// # Examples
    ///
    /// ```
    /// # use stats_ci::*;
    /// let data_a = [1., 2., 3.];
    /// let data_b = [4., 5., 6.];
    /// let ci = comparison::Unpaired::ci(Confidence::new_two_sided(0.95), &data_a, &data_b)?;
    /// # Ok::<(),error::CIError>(())
    /// ```
    ///
    /// # References
    ///
    /// * R. Jain, The Art of Computer Systems Performance Analysis, Wiley, 1991.
    /// * [Wikipedia article on Student's t-test](https://en.wikipedia.org/wiki/Student%27s_t-test#Independent_two-sample_t-test)
    /// * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
    ///
    pub fn ci(confidence: Confidence, data_a: &[T], data_b: &[T]) -> CIResult<Interval<T>> {
        let mut stats = Self::default();
        stats.extend(data_a.into_iter().copied(), data_b.into_iter().copied())?;
        stats.ci_mean(confidence)
    }
}

impl<F: Float> std::ops::Add<Self> for Unpaired<F> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            stats_a: self.stats_a + rhs.stats_a,
            stats_b: self.stats_b + rhs.stats_b,
        }
    }
}

///
/// Given two independent samples, compute the confidence interval of the difference between their means.
/// Unlike [`paired_ci`], the two samples do not have to have the same length.
/// However, comparing with unpaired observations typically requires considerably more observations to
/// reach the same conclusions.
///
/// # Arguments
///
/// * `confidence` - the confidence level
/// * `data_a` - the first sample
/// * `data_b` - the second sample
///
/// # Returns
///
/// The confidence interval of the difference as a result.
///
/// # Notes
///
/// If the interval includes zero, the difference is not significant.
/// If the interval is strictly positive (resp. negative), the mean of the first sample is significantly
/// greater (resp. smaller) than the mean of the second sample.
///
/// [`paired_ci`]: fn.paired_ci.html
///
/// # References
///
/// * R. Jain, The Art of Computer Systems Performance Analysis, Wiley, 1991.
/// * [Wikipedia article on Student's t-test](https://en.wikipedia.org/wiki/Student%27s_t-test#Independent_two-sample_t-test)
/// * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
///
pub fn unpaired_ci<T: Float>(
    confidence: Confidence,
    data_a: &[T],
    data_b: &[T],
) -> CIResult<Interval<T>> {
    let stats_a = mean::Arithmetic::from_iter(data_a.iter().copied())?;
    let stats_b = mean::Arithmetic::from_iter(data_b.iter().copied())?;

    let n_a = T::from(stats_a.sample_count()).convert("stats_a.sample_count")?;
    let n_b = T::from(stats_b.sample_count()).convert("stats_b.sample_count")?;
    let mean_a = stats_a.sample_mean();
    let mean_b = stats_b.sample_mean();
    let std_dev_a = stats_a.sample_std_dev();
    let std_dev_b = stats_b.sample_std_dev();

    let mean_difference = mean_a - mean_b;
    let sa2_na = // $s_a^2 / n_a$
        std_dev_a * std_dev_a / n_a;
    let sb2_nb = // $s_b^2 / n_b$
        std_dev_b * std_dev_b / n_b;
    let sum_s2_n = // $s_a^2 / n_a + s_b^2 / n_b$
        sa2_na + sb2_nb;
    let std_err_mean = // $\sqrt{s_a^2 / n_a + s_b^2 / n_b}$
        sum_s2_n.sqrt();
    let effective_dof = // $ \frac{ (s_a^a / n_a + s_b^2 / n_b)^2 }{ \frac{1}{n_a+1} \left(\frac{s_a^2}{n_a}\right)^2 + \frac{1}{n_b+1} \left(\frac{s_b^2}{n_b}\right)^2 } - 2$
        sum_s2_n * sum_s2_n
            / (sa2_na * sa2_na / (n_a + T::one())
                + sb2_nb * sb2_nb / (n_b + T::one())) - T::one() - T::one();

    let (lo, hi) = stats::interval_bounds(
        confidence,
        mean_difference.try_f64("mean_difference")?,
        std_err_mean.try_f64("std_err_mean")?,
        effective_dof.try_f64("effective_dof")?,
    );
    let lo = T::from(lo).convert("lo")?;
    let hi = T::from(hi).convert("hi")?;
    match confidence {
        Confidence::TwoSided(_) => Interval::new(lo, hi).map_err(|e| e.into()),
        Confidence::UpperOneSided(_) => Ok(Interval::new_upper(lo)),
        Confidence::LowerOneSided(_) => Ok(Interval::new_lower(hi)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_paired() {
        {
            // Case 1
            // based on example from https://online.stat.psu.edu/stat500/lesson/7/7.3/7.3.2

            // Zinc concentration in water samples from a river
            let data_bottom_water = [
                0.430, 0.266, 0.567, 0.531, 0.707, 0.716, 0.651, 0.589, 0.469, 0.723,
            ];
            let data_surface_water = [
                0.415, 0.238, 0.390, 0.410, 0.605, 0.609, 0.632, 0.523, 0.411, 0.612,
            ];

            let ci = paired_ci(
                Confidence::new_two_sided(0.95),
                &data_bottom_water,
                &data_surface_water,
            )
            .unwrap();

            println!("ci = {} (ref: )", ci);
            println!("reference: (0.04299, 0.11781)");
            assert_abs_diff_eq!(ci, Interval::new(0.04299, 0.11781).unwrap(), epsilon = 1e-4);
        }
        {
            // Case 2
            // based on example from https://www.khanacademy.org/math/ap-statistics/xfb5d8e68:inference-quantitative-means/one-sample-t-interval-mean/a/one-sample-t-interval-paired-data

            let data_watch_a = [9.8, 9.8, 10.1, 10.1, 10.2];
            let data_watch_b = [10.1, 10., 10.2, 9.9, 10.1];
            let ci = paired_ci(
                Confidence::new_two_sided(0.95),
                &data_watch_b,
                &data_watch_a,
            )
            .unwrap();

            println!("ci = {}", ci);
            println!("reference: (-0.20, 0.32)");
            assert_abs_diff_eq!(ci, Interval::new(-0.20, 0.32).unwrap(), epsilon = 1e-2);

            let data_pre = [140., 152., 153., 159., 150., 146.];
            let data_post = [150., 159., 170., 164., 148., 166.];
            let ci = paired_ci(Confidence::new_two_sided(0.95), &data_post, &data_pre).unwrap();

            println!("ci = {}", ci);
            println!("reference: (1.03,17.97)");
            assert_abs_diff_eq!(ci, Interval::new(1.03, 17.97).unwrap(), epsilon = 1e-2);
        }
    }

    #[test]
    fn test_unpaired() {
        // based on example from https://www.statsdirect.co.uk/help/parametric_methods/utt.htm
        // itself based on Armitage P, Berry G. Statistical Methods in Medical Research (3rd edition). Blackwell 1994.
        // Consider the gain in weight of 19 female rats between 28 and 84 days after birth. 12 were fed on a high protein diet and 7 on a low protein diet.
        let data_high_protein = [
            134., 146., 104., 119., 124., 161., 107., 83., 113., 129., 97., 123.,
        ];
        let data_low_protein = [70., 118., 101., 85., 107., 132., 94.];
        let ci = unpaired_ci(
            Confidence::new_two_sided(0.95),
            &data_high_protein,
            &data_low_protein,
        )
        .unwrap();

        println!("ci = {}", ci);
        println!("reference: (-2.193679, 40.193679)");
        assert_abs_diff_eq!(
            ci,
            Interval::new(-2.193679, 40.193679).unwrap(),
            epsilon = 1e-2
        );
    }

    #[test]
    fn test_incremental_unpaired() {
        let sample_size_a = 100;
        let sample_size_b = 200;
        let data_a = (0..sample_size_a)
            .map(|_| rand::random::<f64>())
            .collect::<Vec<_>>();
        let data_b = (0..sample_size_b)
            .map(|_| rand::random::<f64>())
            .collect::<Vec<_>>();
        let confidence = Confidence::new_two_sided(0.95);

        let ref_ci = comparison::unpaired_ci(confidence, &data_a, &data_b).unwrap();
        let wrap_ci = comparison::Unpaired::ci(confidence, &data_a, &data_b).unwrap();
        let mut stats = comparison::Unpaired::default();
        stats.extend(data_a, data_b).unwrap();
        let other_ci = stats.ci_mean(confidence).unwrap();

        println!("ref_ci   = {}", ref_ci);
        println!("other_ci = {}", other_ci);
        println!("wrap_ci  = {}", wrap_ci);
        assert_eq!(ref_ci, other_ci);
        assert_eq!(ref_ci, wrap_ci);
    }

    #[test]
    fn test_incremental_paired() {
        let sample_size = 100;
        let data1 = (0..sample_size)
            .map(|_| rand::random::<f64>())
            .collect::<Vec<_>>();
        let data2 = (0..sample_size)
            .map(|_| rand::random::<f64>())
            .collect::<Vec<_>>();
        let confidence = Confidence::new_two_sided(0.95);

        let ref_ci = comparison::paired_ci(confidence, &data1, &data2).unwrap();
        let wrap_ci = comparison::Paired::ci(confidence, &data1, &data2).unwrap();
        let mut stats = comparison::Paired::default();
        stats.extend(data1, data2).unwrap();
        let other_ci = stats.ci_mean(confidence).unwrap();

        println!("ref_ci   = {}", ref_ci);
        println!("other_ci = {}", other_ci);
        println!("wrap_ci  = {}", wrap_ci);
        assert_eq!(ref_ci, other_ci);
        assert_eq!(ref_ci, wrap_ci);

        {
            // Case 1
            // based on example from https://online.stat.psu.edu/stat500/lesson/7/7.3/7.3.2

            // Zinc concentration in water samples from a river
            let data_bottom_water = [
                0.430, 0.266, 0.567, 0.531, 0.707, 0.716, 0.651, 0.589, 0.469, 0.723,
            ];
            let data_surface_water = [
                0.415, 0.238, 0.390, 0.410, 0.605, 0.609, 0.632, 0.523, 0.411, 0.612,
            ];

            let mut stats = comparison::Paired::default();
            stats.extend(data_bottom_water, data_surface_water).unwrap();
            let ci = stats.ci_mean(Confidence::new_two_sided(0.95)).unwrap();

            println!("ci = {} (ref: )", ci);
            println!("reference: (0.04299, 0.11781)");
            assert_abs_diff_eq!(ci, Interval::new(0.04299, 0.11781).unwrap(), epsilon = 1e-4);
        }
    }

    #[test]
    fn test_paired_diff_length() {
        let sample_size = 10;
        let data1 = (0..sample_size)
            .map(|_| rand::random::<f64>())
            .collect::<Vec<_>>();
        let data2 = (0..sample_size + 1)
            .map(|_| rand::random::<f64>())
            .collect::<Vec<_>>();

        let mut stats = comparison::Paired::default();
        let res = stats.extend(data1, data2);
        assert!(res.is_err());
        match res.unwrap_err() {
            CIError::DifferentSampleSizes(a, b) => {
                println!("DifferentSampleSizes({a},{b})");
                assert_eq!(a, sample_size);
                assert_eq!(b, sample_size + 1);
            }
            e => panic!("unexpected error: {}", e),
        }
    }
}
