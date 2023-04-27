use crate::*;

use error::*;
use num_traits::Float;

///
/// Experimental: not tested yet.
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
/// # Panics
///
/// * if the two samples have different lengths
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
///
pub fn paired_ci<T: Float>(
    confidence: Confidence,
    data1: &[T],
    data2: &[T],
) -> CIResult<Interval<T>> {
    assert!(data1.len() == data2.len());

    let data = data1.iter().zip(data2).map(|(&x, &y)| x - y);
    mean::Arithmetic::ci(confidence, data)
}

///
/// Experimental: not tested yet.
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
///
pub fn unpaired_ci<T: Float>(
    confidence: Confidence,
    data_a: &[T],
    data_b: &[T],
) -> CIResult<Interval<T>> {
    let stats_a = utils::sample_len_mean_stddev_with_transform(data_a, |_| true, |&x| x)?;
    let stats_b = utils::sample_len_mean_stddev_with_transform(data_b, |_| true, |&y| y)?;

    let mean_difference = stats_a.mean - stats_b.mean;
    let sa2_na = // $s_a^2 / n_a$
        stats_a.std_dev * stats_a.std_dev / stats_a.n;
    let sb2_nb = // $s_b^2 / n_b$
        stats_b.std_dev * stats_b.std_dev / stats_b.n;
    let sum_s2_n = // $s_a^2 / n_a + s_b^2 / n_b$
        sa2_na + sb2_nb;
    let std_err_mean = // $\sqrt{s_a^2 / n_a + s_b^2 / n_b}$
        (sum_s2_n).sqrt();
    let effective_dof = // $ \frac{ (s_a^a / n_a + s_b^2 / n_b)^2 }{ \frac{1}{n_a+1} \left(\frac{s_a^2}{n_a}\right)^2 + \frac{1}{n_b+1} \left(\frac{s_b^2}{n_b}\right)^2 } - 2$
        sum_s2_n * sum_s2_n
            / (sa2_na * sa2_na / (stats_a.n + T::one())
                + sb2_nb * sb2_nb / (stats_b.n + T::one())) - T::one() - T::one();
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
