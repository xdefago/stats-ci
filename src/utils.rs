use crate::*;
use error::*;
use num_traits::Float;

///
/// compensated Kahan summation.
/// See <https://en.wikipedia.org/wiki/Kahan_summation_algorithm>
///
/// The function is meant to be called at each iteration of the summation,
/// with relevant variables managed externally
///
/// # Arguments
///
/// * `current_sum` - the current sum
/// * `x` - the next value to add to the sum
/// * `compensation` - the compensation term
///
pub(crate) fn kahan_add<T: Float>(current_sum: &mut T, x: T, compensation: &mut T) {
    let sum = *current_sum;
    let c = *compensation;
    let y = x - c;
    let t = sum + y;
    *compensation = (t - sum) - y;
    *current_sum = t;
}

///
/// Holds the statistics of a sample.
///
/// # Fields
///
/// * `len` - the length of the sample
/// * `n` - the length of the sample as the type used for the computation
/// * `mean` - the mean of the sample as the type used for the computation
/// * `std_dev` - the standard deviation of the sample as the type used for the computation
///
#[derive(Debug, Clone)]
pub(crate) struct Statistics<T: Float> {
    pub len: usize,
    pub n: T,
    pub mean: T,
    pub std_dev: T,
}

///
/// Compute the mean and standard deviation of a sample.
/// The sample is given as an iterator, and the function
/// takes two closures to filter out invalid values and
/// transform the values into the type used for the computation.
///
/// # Arguments
///
/// * `data` - the sample data
/// * `f_valid` - a closure that returns true if the value is valid
/// * `f_transform` - a closure that transforms the value into the type used for the computation
///
/// # Returns
///
/// A result with the following tuple (when `Ok(_)`):
/// * the length of the sample (as `usize`)
/// * the length of the sample as the type used for the computation
/// * the mean of the sample as the type used for the computation
/// * the standard deviation of the sample as the type used for the computation
///
/// # Errors
///
/// * `CIError::InvalidInputData` - if the sample contains invalid values
/// * `CIError::FloatConversionError` - if the sample length cannot be converted into the type used for the computation
///
/// # Notes
///
/// This uses a naive implementation of the variance/standard deviation which is normally numerically unstable.
/// However, when comparing it against results from a more stable implementation on random data of 100 millions samples in f32, the relative difference is negligible (less than 1e-6%).
pub(crate) fn sample_len_mean_stddev_with_transform<T, U, I, Fvalid, Ftransform>(
    data: I,
    f_valid: Fvalid,
    f_transform: Ftransform,
) -> CIResult<Statistics<U>>
where
    T: PartialOrd,
    U: Float,
    I: IntoIterator<Item = T>,
    Fvalid: Fn(&T) -> bool,
    Ftransform: Fn(T) -> U,
{
    let mut len = 0_usize;
    let mut sum = <U>::zero();
    let mut sum_c = <U>::zero();
    let mut sum_sq = <U>::zero();
    let mut sum_sq_c = <U>::zero();
    for x in data {
        if !f_valid(&x) {
            return Err(error::CIError::InvalidInputData);
        }
        let x_prime = f_transform(x);
        kahan_add(&mut sum, x_prime, &mut sum_c);
        kahan_add(&mut sum_sq, x_prime * x_prime, &mut sum_sq_c);
        len += 1;
    }
    let n = U::from(len).convert("len")?;

    let mean = sum / n;
    let variance = (sum_sq - sum * mean) / (n - U::one());
    let std_dev = variance.sqrt();
    Ok(Statistics {
        len,
        n,
        mean,
        std_dev,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;
    use rand::distributions::Distribution;
    use statrs::distribution::*;

    use rand_chacha::ChaCha8Rng;
    use rand_seeder::Seeder;

    #[test]
    fn test_kahan_add() {
        type Float = f32;
        let iterations = 50_000_000_usize;
        let mut normal: Float = 0.;
        let mut kahan: Float = 0.;
        let mut kahan_c: Float = 0.;
        let x = 1.1;

        for _ in 0..iterations {
            normal += x;
            kahan_add(&mut kahan, x, &mut kahan_c);
        }
        let expected = iterations as Float * x;
        println!("should be: {}", expected);
        println!(
            "normal: {} (diff: {:.0}%)",
            normal,
            (normal - expected) / expected * 100.
        );
        println!(
            "kahan: {} (diff: {:.0}%)",
            kahan,
            (kahan - expected) / expected * 100.
        );
        assert_approx_eq!(expected, kahan, 1e-10);
        assert!((expected - normal).abs() > 500_000.); // normal summation is not accurate for f32
    }

    #[test]
    fn test_compare_accuracy() {
        use statrs::statistics::Distribution;
        use std::time::Instant;
        type Float = f64;

        const SEED_STRING: &str = "this is an RNG seed used in test_compare_accuracy()";
        let mut rng: ChaCha8Rng = Seeder::from(SEED_STRING).make_rng();
        let sample_size = 10_000_000_usize;
        let distribution = Normal::new(0., 1.).unwrap();

        let data: Vec<Float> = (0..sample_size)
            .map(|_| distribution.sample(&mut rng) as Float)
            .collect();

        let data_a = data;

        let point_a = Instant::now();
        let tuple_a = sample_len_mean_stddev_with_transform(data_a, |_| true, |x| x).unwrap();
        let point_b = Instant::now();

        let distrib_mean = distribution.mean().unwrap() as Float;
        let distrib_stddev = distribution.std_dev().unwrap() as Float;

        println!("tuple_a (naive): {:?}", tuple_a);
        println!(
            "deviation from distribution mean: {}",
            (tuple_a.mean - distrib_mean)
        );
        println!(
            "deviation from distribution stddev: {} %",
            (tuple_a.std_dev - distrib_stddev) / distrib_stddev * 100.
        );
        println!("time: {} ms", point_b.duration_since(point_a).as_millis());

        assert!((tuple_a.mean - distrib_mean).abs() < 1e-4);
        assert!(((tuple_a.std_dev - distrib_stddev) / distrib_stddev).abs() < 1e-3);
    }
}
