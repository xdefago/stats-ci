use crate::*;

use lazy_static::lazy_static;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::{Normal, StudentsT};

///
/// return the z-value of the normal distribution for a given confidence level.
///
/// # Arguments
///
/// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
///
pub fn z_value(confidence: Confidence) -> f64 {
    lazy_static! {
        static ref NORMAL: Normal = Normal::new(0., 1.).unwrap();
    }
    NORMAL.inverse_cdf(confidence.quantile())
}

///
/// return the t-value of the t-distribution for a given confidence level and degree of freedom.
///
/// # Arguments
///
/// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
/// * `degrees_of_freedom` - the degrees of freedom of the t-distribution
///
/// # Panics
///
/// * if `degrees_of_freedom` is negative or zero
///
pub fn t_value(confidence: Confidence, degrees_of_freedom: f64) -> f64 {
    let student_t = StudentsT::new(0., 1., degrees_of_freedom).unwrap();
    student_t.inverse_cdf(confidence.quantile())
}

const POPULATION_LIMIT: f64 = 100_000.;

pub(crate) fn interval_bounds(
    confidence: Confidence,
    mean: f64,
    std_err_mean: f64,
    degrees_of_freedom: f64,
) -> (f64, f64) {
    let span = if degrees_of_freedom < POPULATION_LIMIT {
        let t = t_value(confidence, degrees_of_freedom);
        t * std_err_mean
    } else {
        let z = z_value(confidence);
        z * std_err_mean
    };
    (mean - span, mean + span)
}

#[cfg(all(test, feature = "approx"))]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_t_and_z_value() {
        for confidence_level in [0.5, 0.8, 0.9, 0.95, 0.99, 0.999] {
            for new_confidence in [
                Confidence::new_two_sided,
                Confidence::new_upper,
                Confidence::new_lower,
            ] {
                let confidence = new_confidence(confidence_level);
                let t_value = t_value(confidence, 1000.);
                let z_value = z_value(confidence);
                assert_abs_diff_eq!(t_value, z_value, epsilon = 1e-2);
            }
        }
    }

    #[test]
    fn test_interval_bounds() {
        let confidence = Confidence::new_two_sided(0.95);
        let mean = 0.;
        let std_err_mean = 1.;

        for n in [
            POPULATION_LIMIT - 1.,
            POPULATION_LIMIT,
            POPULATION_LIMIT + 1.,
            POPULATION_LIMIT + 2.,
        ] {
            let degrees_of_freedom = n - 1.;
            let actual = interval_bounds(confidence, mean, std_err_mean, degrees_of_freedom);
            let bounds_t = only_t(confidence, mean, std_err_mean, degrees_of_freedom);
            let bounds_z = only_z(confidence, mean, std_err_mean);

            println!("n = {} (dof: {})", n, degrees_of_freedom);
            println!("actual: ({}, {})", actual.0, actual.1);
            println!("bounds_t: ({}, {})", bounds_t.0, bounds_t.1);
            println!("bounds_z: ({}, {})", bounds_z.0, bounds_z.1);
            println!();

            assert_abs_diff_eq!(actual.0, bounds_t.0, epsilon = 1e-4);
            assert_abs_diff_eq!(actual.1, bounds_t.1, epsilon = 1e-4);
            assert_abs_diff_eq!(actual.0, bounds_z.0, epsilon = 1e-4);
            assert_abs_diff_eq!(actual.1, bounds_z.1, epsilon = 1e-4);
        }
    }

    fn only_t(
        confidence: Confidence,
        mean: f64,
        std_err_mean: f64,
        degrees_of_freedom: f64,
    ) -> (f64, f64) {
        let t = t_value(confidence, degrees_of_freedom);
        let span = t * std_err_mean;
        (mean - span, mean + span)
    }
    fn only_z(confidence: Confidence, mean: f64, std_err_mean: f64) -> (f64, f64) {
        let z = z_value(confidence);
        let span = z * std_err_mean;
        (mean - span, mean + span)
    }
}
