//! Confidence intervals (two sided) over the mean of a given sample
//!
//! # Examples
//!
//! ```
//! # use stats_ci::mean;
//! let data = [
//!     82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
//!     15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
//!     71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
//!     98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
//!     49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
//!     37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
//! ];
//! let ci = mean::ci(0.95, data).unwrap();
//! // mean: 53.67
//! // stddev: 28.097613040716798
//!
//! use num_traits::Float;
//! use assert_approx_eq::assert_approx_eq;
//! assert_approx_eq!(ci.low().unwrap(), 48.0948, 1e-3);
//! assert_approx_eq!(ci.high().unwrap(), 59.2452, 1e-3);
//! ```
use super::*;

use error::*;
use num_traits::Float;

///
/// computes the two-sided confidence interval over the arithmetic mean of a given sample
///
/// # Arguments
///
/// * `data` - the sample data
/// * `confidence` - the confidence level (must be in (0, 1))
///
/// # Errors
///
/// * `TooFewSamples` - if the sample size is too small to compute a confidence interval
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
///
/// # Examples
///
/// ```
/// # use stats_ci::mean;
/// let data = [
///     82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
///     15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
///     71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
///     98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
///     49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
///     37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
/// ];
/// let ci = mean::ci(0.95, data).unwrap();
/// // mean: 53.67
/// // stddev: 28.097613040716798
///
/// use num_traits::Float;
/// use assert_approx_eq::assert_approx_eq;
/// assert_approx_eq!(ci.low().unwrap(), 48.0948, 1e-3);
/// assert_approx_eq!(ci.high().unwrap(), 59.2452, 1e-3);
/// ```
///
pub fn ci<T, I>(confidence: f64, data: I) -> CIResult<Interval<T>>
where
    T: Float,
    I: IntoIterator<Item = T>,
{
    let mut sum = T::zero();
    let mut sum_sq = T::zero();
    let population = data
        .into_iter()
        .inspect(|&x| sum = sum + x)
        .inspect(|&x| sum_sq = sum_sq + x * x)
        .count();
    if population < 2 {
        return Err(CIError::TooFewSamples(population));
    }

    let z_or_t = T::max(
        // z value from normal distribution
        T::from(z_value(confidence)).unwrap(),
        // t value from student's t distribution
        T::from(t_value(confidence, population - 1)).unwrap(),
    );
    let n = T::from(population).unwrap();

    let mean = sum / n;
    let variance = (sum_sq - sum * sum / n) / (n - T::one());
    let std_dev = variance.sqrt();
    Ok(Interval::new(
        mean - z_or_t * std_dev / n.sqrt(),
        mean + z_or_t * std_dev / n.sqrt(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_mean_ci() {
        let data = [
            82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
            15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
            71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
            98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
            49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
            37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
        ];
        let ci = ci(0.95, data).unwrap();
        // mean: 53.67
        // stddev: 28.097613040716798
        assert_approx_eq!(ci.low().unwrap(), 48.0948, 1e-3);
        assert_approx_eq!(ci.high().unwrap(), 59.2452, 1e-3);
        assert_approx_eq!(ci.low().unwrap() + ci.high().unwrap(), 2. * 53.67, 1e-3);
    }

    #[test]
    fn test_confidence_level() {
        type Float = f64;
        use rand::Rng;

        let mut rng = rand::thread_rng();

        const POPULATION_SIZE: usize = 10_000;
        let repetitions = 10_000;
        let sample_size = 10;
        let confidence = 0.95;
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
            let sample_ci = ci(confidence, sample).unwrap();
            if sample_ci.contains(&population_mean) {
                count_in_ci += 1;
            }
        }
        let ci_contains_mean = count_in_ci as f64 / repetitions as f64;
        assert_approx_eq!(ci_contains_mean, confidence, tolerance);
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
