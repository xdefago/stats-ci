//! Confidence intervals (two sided) over the mean of a given sample
//!
//! # Examples
//!
//! Confidence intervals on the arithmetic mean of a sample:
//! ```
//! # fn test() -> Result<(), stats_ci::error::CIError> {
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
//! assert_approx_eq!(ci.low().unwrap(), 48.0948, 1e-3);
//! assert_approx_eq!(ci.high().unwrap(), 59.2452, 1e-3);
//! # Ok(())
//! # }
//! ```
//!
//! Confidence intervals on the geometric mean of a sample:
//! ```
//! # fn test() -> Result<(), stats_ci::error::CIError> {
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
//! assert_approx_eq!(ci.low().unwrap(), 37.731, 1e-3);
//! assert_approx_eq!(ci.high().unwrap(), 50.675, 1e-3);
//! # Ok(())
//! # }
//! ```
//!
//! Confidence intervals on the harmonic mean of a sample:
//! ```
//! # fn test() -> Result<(), stats_ci::error::CIError> {
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
//! assert_approx_eq!(ci.low().unwrap(), 0.245, 1e-3);
//! assert_approx_eq!(ci.high().unwrap(), 0.852, 1e-3);
//! # Ok(())
//! # }
//! ```
//!
use super::*;

use error::*;
use num_traits::Float;

///
/// Trait for computing confidence intervals on the mean of a sample.
///
/// # Examples
///
/// ```
/// # fn test() -> Result<(), stats_ci::error::CIError> {
/// use stats_ci::*;
/// let data = [
///    82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
///   15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
///  71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
/// 98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
/// 49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
/// 37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
/// ];
/// let confidence = Confidence::new_two_sided(0.95);
/// let ci = mean::Arithmetic::ci(confidence, data)?;
/// // arithmetic mean: 52.5
///
/// use num_traits::Float;
/// use assert_approx_eq::assert_approx_eq;
/// assert_approx_eq!(ci.low().unwrap(), 47.5, 1e-3);
/// assert_approx_eq!(ci.high().unwrap(), 57.5, 1e-3);
/// # Ok(())
/// # }
/// ```
pub trait MeanCI<T> {
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
            |x, y| (x, y),
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
            |x, y| (x.exp(), y.exp()),
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
            |x| x.recip(),                 // 1/x
            |x, y| (y.recip(), x.recip()), // NB: bounds are mirrored
        )
    }
}

fn ci_with_transforms<T: PartialOrd, U: Float, I, F, Finv, Fvalid>(
    confidence: Confidence,
    data: I,
    f_valid: Fvalid,
    f_transform: F,
    f_inverse: Finv,
) -> CIResult<Interval<T>>
where
    I: IntoIterator<Item = T>,
    Fvalid: Fn(&T) -> bool,
    F: Fn(T) -> U,
    Finv: Fn(U, U) -> (T, T),
{
    use itertools::Itertools;

    let mut sum = U::zero();
    let mut sum_sq = U::zero();
    let population = data
        .into_iter()
        // test validitity and return InvalidInputData
        .map(|x| {
            if f_valid(&x) {
                Ok(x)
            } else {
                Err(CIError::InvalidInputData)
            }
        })
        // count population size and compute sum and sum of squares
        .fold_ok(0_usize, |acc, x| {
            let x_prime = f_transform(x);
            sum = sum + x_prime;
            sum_sq = sum_sq + x_prime * x_prime;
            acc + 1
        })?;
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
            std::any::type_name::<T>()
        ))
    })?;

    let mean = sum / n;
    let variance = (sum_sq - sum * sum / n) / (n - U::one());
    let std_dev = variance.sqrt();
    Ok(Interval::from(f_inverse(
        mean - t_value * std_dev / n.sqrt(),
        mean + t_value * std_dev / n.sqrt(),
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_mean_ci() {
        let confidence = Confidence::new_two_sided(0.95);
        let data = [
            82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
            15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
            71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
            98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
            49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
            37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
        ];
        let ci = Arithmetic::ci(confidence, data).unwrap();
        // mean: 53.67
        // stddev: 28.097613040716798
        assert_approx_eq!(ci.low().unwrap(), 48.0948, 1e-3);
        assert_approx_eq!(ci.high().unwrap(), 59.2452, 1e-3);
        assert_approx_eq!(ci.low().unwrap() + ci.high().unwrap(), 2. * 53.67, 1e-3);
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
        assert_approx_eq!(ci.low().unwrap(), 0.245, 1e-3);
        assert_approx_eq!(ci.high().unwrap(), 0.852, 1e-3);
    }

    #[test]
    fn test_confidence_level() {
        type Float = f64;
        use rand::Rng;

        let mut rng = rand::thread_rng();

        const POPULATION_SIZE: usize = 10_000;
        let repetitions = 10_000;
        let sample_size = 10;
        let confidence = Confidence::new_two_sided(0.95);
        let tolerance = 0.02;

        // generate population (uniformly distributed between 0 and 1)
        let mut population = [0 as Float; POPULATION_SIZE];
        rng.fill(&mut population[..]);
        let population_mean = population.iter().sum::<Float>() / POPULATION_SIZE as Float;
        println!("population_mean: {}", population_mean);
        println!("population head: {:?}", &population[..10]);

        // generate samples and compute confidence intervals
        let mut count_in_ci = 0;
        for _ in 0..repetitions {
            // generate sample
            let sample = random_sample(&population, sample_size, &mut rng);
            let sample_ci = Arithmetic::ci(confidence, sample).unwrap();
            if sample_ci.contains(&population_mean) {
                count_in_ci += 1;
            }
        }
        let ci_contains_mean = count_in_ci as f64 / repetitions as f64;
        assert_approx_eq!(ci_contains_mean, confidence.level(), tolerance);
    }

    fn random_sample<T: Copy>(
        data: &[T],
        sample_size: usize,
        rng: &mut rand::rngs::ThreadRng,
    ) -> Vec<T> {
        use rand::Rng;
        assert!(sample_size < data.len());

        (0..sample_size)
            .map(|_| rng.gen_range(0..data.len()))
            .map(|i| data[i])
            .collect()
    }
}
