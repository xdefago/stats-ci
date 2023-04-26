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
    // iterate through the data and compute the sample size, mean, and standard deviation.
    // applies the validity and transformation functions to the data.
    let stats = utils::sample_len_mean_stddev_with_transform(data, f_valid, f_transform)?;

    // use the t-distribution regardless of the population size
    let t = U::from(t_value(confidence, stats.len - 1)).ok_or_else(|| {
        CIError::FloatConversionError(format!(
            "converting t-value into type {}",
            std::any::type_name::<T>()
        ))
    })?;

    let span = t * stats.std_dev / stats.n.sqrt();
    match confidence {
        Confidence::TwoSided(_) => {
            let low = if !flipped {
                f_inverse(stats.mean - span)
            } else {
                f_inverse(stats.mean + span)
            };
            let high = if !flipped {
                f_inverse(stats.mean + span)
            } else {
                f_inverse(stats.mean - span)
            };
            Interval::new(low, high).map_err(|e| e.into())
        }
        Confidence::UpperOneSided(_) => {
            let low = if !flipped {
                f_inverse(stats.mean - span)
            } else {
                f_inverse(stats.mean + span)
            };
            Ok(Interval::new_upper(low))
        }
        Confidence::LowerOneSided(_) => {
            let high = if !flipped {
                f_inverse(stats.mean + span)
            } else {
                f_inverse(stats.mean - span)
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

        Ok(())
    }

    #[test]
    fn test_harmonic_ci() -> CIResult<()>  {
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

        Ok(())
    }
}
