//! Comparison of two samples
//! ========================
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
/// * PennState. Stat 500. Lesson 7: Comparing Two Population Parameters. [Online](https://online.stat.psu.edu/stat500/lesson/7)
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
    use assert_approx_eq::assert_approx_eq;

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
            assert_approx_eq!(ci.low_f(), 0.04299, 1e-4);
            assert_approx_eq!(ci.high_f(), 0.11781, 1e-4);
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
            assert_approx_eq!(ci.low_f(), -0.20, 1e-2);
            assert_approx_eq!(ci.high_f(), 0.32, 1e-2);

            let data_pre = [140., 152., 153., 159., 150., 146.];
            let data_post = [150., 159., 170., 164., 148., 166.];
            let ci = paired_ci(Confidence::new_two_sided(0.95), &data_post, &data_pre).unwrap();

            println!("ci = {}", ci);
            println!("reference: (1.03,17.97)");
            assert_approx_eq!(ci.low_f(), 1.03, 1e-2);
            assert_approx_eq!(ci.high_f(), 17.97, 1e-2);
        }
    }

    #[test]
    fn test_unpaired() {
        // based on example from https://www.statsdirect.co.uk/help/parametric_methods/utt.htm
        // itself based on Armitage P, Berry G. Statistical Methods in Medical Research (3rd edition). Blackwell 1994.
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
        assert_approx_eq!(ci.low_f(), -2.193679, 1e-2);
        assert_approx_eq!(ci.high_f(), 40.193679, 1e-2);
    }
}
