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
//!
//! use num_traits::Float;
//! use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low_f(), 41.6496, 1e-3);
//! assert_approx_eq!(ci.high_f(), 65.69, 1e-3);
//! # Ok(())
//! # }
//! ```
//!
//! Confidence intervals on the geometric mean of a sample:
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
//! let ci = mean::Geometric::ci(confidence, data)?;
//! // geometric mean: 43.7268032829256
//!
//! use num_traits::Float;
//! use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low_f(), 20.368, 1e-3);
//! assert_approx_eq!(ci.high_f(), 93.875, 1e-3);
//! # Ok(())
//! # }
//! ```
//!
//! Confidence intervals on the harmonic mean of a sample:
//! ```
//! # fn test() -> stats_ci::CIResult<()> {
//! use stats_ci::*;
//! let data = [
//!     1.81600583, 0.07498389, 1.29092744, 0.62023863, 0.09345327, 1.94670997, 2.27687339,
//!     0.9251231, 1.78173864, 0.4391542, 1.36948099, 1.5191194, 0.42286756, 1.48463176,
//!     0.17621009, 2.31810064, 0.15633061, 2.55137878, 1.11043948, 1.35923319, 1.58385561,
//!     0.63431437, 0.49993148, 0.49168534, 0.11533354,
//! ];
//! let confidence = Confidence::new_two_sided(0.95);
//! let ci = mean::Harmonic::ci(confidence, data.clone())?;
//! // harmonic mean: 0.38041820166550844
//!
//! use num_traits::Float;
//! use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low_f(), 0.225, 1e-3);
//! assert_approx_eq!(ci.high_f(), 1.23, 1e-3);
//! # Ok(())
//! # }
//! ```
//!
use super::*;
use crate::stats::t_value;
use crate::utils;

use error::*;
use num_traits::Float;

///
/// Trait for computing confidence intervals on the mean of a sample.
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
pub trait MeanCI<T: PartialOrd> {
    fn ci<I>(confidence: Confidence, data: I) -> CIResult<Interval<T>>
    where
        I: IntoIterator<Item = T>;
}

///
/// Computation for arithmetic mean.
///
pub struct Arithmetic;

impl<T: Float> MeanCI<T> for Arithmetic {
    fn ci<I>(confidence: Confidence, data: I) -> CIResult<Interval<T>>
    where
        I: IntoIterator<Item = T>,
    {
        ci_with_transforms(
            confidence,
            data,
            |x: &T| !x.is_nan() && !x.is_infinite(),
            |x| x,
            |x| x,
            false,
        )
    }
}

///
/// Computation for geometric mean.
///
pub struct Geometric;

impl<T: Float> MeanCI<T> for Geometric {
    fn ci<I>(confidence: Confidence, data: I) -> CIResult<Interval<T>>
    where
        I: IntoIterator<Item = T>,
    {
        ci_with_transforms(
            confidence,
            data,
            |x: &T| x.is_sign_positive() || !x.is_zero(),
            |x| x.ln(),
            |x| x.exp(),
            false,
        )
    }
}

///
/// Computation for harmonic mean.
///
pub struct Harmonic;

impl<T: Float> MeanCI<T> for Harmonic {
    fn ci<I>(confidence: Confidence, data: I) -> CIResult<Interval<T>>
    where
        I: IntoIterator<Item = T>,
    {
        ci_with_transforms(
            confidence,
            data,
            |x: &T| x.is_sign_positive() || !x.is_zero(),
            |x| x.recip(), // 1/x
            |x| x.recip(),
            true,
        )
    }
}

///
/// Compute the confidence interval for the mean of a sample,
/// applying validity and transformation functions to the sample data.
///
/// # Arguments
///
/// * `confidence` - the confidence level
/// * `data` - the sample data
/// * `f_valid` - a function to determine whether a value is valid
/// * `f_transform` - a function to transform a value before computing the mean
/// * `f_inverse` - the inverse function to transform the bounds of the confidence interval
/// * `flipped` - whether the confidence interval is flipped by the transformation (i.e. the lower bound is the upper bound)
///
/// # Errors
///
/// * `CIError::InvalidInputData` - if the sample data is empty or contains invalid values
/// * `CIError::InvalidTooFewSamples` - if the sample size is not sufficient
/// * `CIError::FloatConversionError` - if the conversion from `T` to `U` fails
///
fn ci_with_transforms<T: PartialOrd, U: Float, I, F, Finv, Fvalid>(
    confidence: Confidence,
    data: I,
    f_valid: Fvalid,
    f_transform: F,
    f_inverse: Finv,
    flipped: bool,
) -> CIResult<Interval<T>>
where
    I: IntoIterator<Item = T>,
    Fvalid: Fn(&T) -> bool,
    F: Fn(T) -> U,
    Finv: Fn(U) -> T,
{
    let mut sum = U::zero();
    let mut sum_c = U::zero(); // compensation for Kahan summation
    let mut sum_sq = U::zero();
    let mut sum_sq_c = U::zero(); // compensation for Kahan summation
    let mut population = 0_usize;

    for x in data {
        if !f_valid(&x) {
            return Err(CIError::InvalidInputData);
        }
        let x_prime = f_transform(x);
        utils::kahan_add(&mut sum, x_prime, &mut sum_c);
        utils::kahan_add(&mut sum_sq, x_prime * x_prime, &mut sum_sq_c);
        population += 1;
    }

    if population < 2 {
        return Err(CIError::TooFewSamples(population));
    }

    // use the t-distribution regardless of the population size
    let t_value = U::from(t_value(confidence, population - 1)).ok_or_else(|| {
        CIError::FloatConversionError(format!(
            "converting t-value into type {}",
            std::any::type_name::<T>()
        ))
    })?;
    let n = U::from(population).ok_or_else(|| {
        CIError::FloatConversionError(format!(
            "converting population ({}) into type {}",
            population,
            std::any::type_name::<U>()
        ))
    })?;

    let mean = sum / n;
    let variance = (sum_sq - mean * mean) / (n - U::one());
    let std_dev = variance.sqrt();
    let span = t_value * std_dev / n.sqrt();
    match confidence {
        Confidence::TwoSided(_) => {
            let low = if !flipped {
                f_inverse(mean - span)
            } else {
                f_inverse(mean + span)
            };
            let high = if !flipped {
                f_inverse(mean + span)
            } else {
                f_inverse(mean - span)
            };
            Interval::new(low, high).map_err(|e| e.into())
        }
        Confidence::UpperOneSided(_) => {
            let low = if !flipped {
                f_inverse(mean - span)
            } else {
                f_inverse(mean + span)
            };
            Ok(Interval::new_upper(low))
        }
        Confidence::LowerOneSided(_) => {
            let high = if !flipped {
                f_inverse(mean + span)
            } else {
                f_inverse(mean - span)
            };
            Ok(Interval::new_lower(high))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_mean_ci() -> CIResult<()> {
        let confidence = Confidence::new_two_sided(0.95);
        let data = [
            82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
            15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
            71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
            98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
            49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
            37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
        ];
        let ci = Arithmetic::ci(confidence, data)?;
        // mean: 53.67
        // stddev: 28.097613040716798
        assert_approx_eq!(ci.low_f(), 41.6496, 1e-3);
        assert_approx_eq!(ci.high_f(), 65.69, 1e-3);
        assert_approx_eq!(ci.low_f() + ci.high_f(), 2. * 53.67, 1e-3);

        let ci2 = Arithmetic::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_approx_eq!(ci2.low_f(), ci.low_f(), 1e-3);
        assert_eq!(ci2.high_f(), f64::INFINITY);
        let ci2 = Arithmetic::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_approx_eq!(ci2.high_f(), ci.high_f(), 1e-3);
        assert_eq!(ci2.low_f(), f64::NEG_INFINITY);

        let ci = Harmonic::ci(confidence, data)?;
        // harmonic mean: 30.031313156339586
        assert_approx_eq!(ci.low_f(), 22.47, 1e-3);
        assert_approx_eq!(ci.high_f(), 45.26, 1e-3);

        let ci2 = Harmonic::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_approx_eq!(ci2.low_f(), ci.low_f(), 1e-6);
        assert_eq!(ci2.high_f(), f64::INFINITY);
        let ci2 = Harmonic::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_approx_eq!(ci2.high_f(), ci.high_f(), 1e-6);
        assert_eq!(ci2.low_f(), f64::NEG_INFINITY);

        let ci = Geometric::ci(confidence, data)?;
        // geometric mean: 43.7268032829256
        assert_approx_eq!(ci.low_f(), 20.368, 1e-3);
        assert_approx_eq!(ci.high_f(), 93.875, 1e-3);

        let ci2 = Geometric::ci(Confidence::UpperOneSided(0.975), data)?;
        assert_approx_eq!(ci2.low_f(), ci.low_f(), 1e-3);
        assert_eq!(ci2.high_f(), f64::INFINITY);
        let ci2 = Geometric::ci(Confidence::LowerOneSided(0.975), data)?;
        assert_approx_eq!(ci2.high_f(), ci.high_f(), 1e-3);
        assert_eq!(ci2.low_f(), f64::NEG_INFINITY);

        Ok(())
    }

    #[test]
    fn test_harmonic_ci() {
        let confidence = Confidence::new_two_sided(0.95);
        let data = [
            1.81600583, 0.07498389, 1.29092744, 0.62023863, 0.09345327, 1.94670997, 2.27687339,
            0.9251231, 1.78173864, 0.4391542, 1.36948099, 1.5191194, 0.42286756, 1.48463176,
            0.17621009, 2.31810064, 0.15633061, 2.55137878, 1.11043948, 1.35923319, 1.58385561,
            0.63431437, 0.49993148, 0.49168534, 0.11533354,
        ];
        let ci = Harmonic::ci(confidence, data).unwrap();
        // harmonic mean: 0.38041820166550844
        assert_approx_eq!(ci.low_f(), 0.225, 1e-3);
        assert_approx_eq!(ci.high_f(), 1.23, 1e-3);
    }

    #[test]
    fn test_kahan_add() {
        type Float = f32;
        let mut normal: Float = 0.;
        let mut kahan: Float = 0.;
        let mut kahan_c: Float = 0.;
        let x = 0.1;

        for _ in 0..50_000_000_usize {
            normal += x;
            utils::kahan_add(&mut kahan, x, &mut kahan_c);
        }

        assert_approx_eq!(5_000_000., kahan, 1e-10);
        assert!((5_000_000. - normal).abs() > 500_000.); // normal summation is not accurate for f32
    }
}
