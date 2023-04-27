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

pub(crate) fn interval_bounds(
    confidence: Confidence,
    mean: f64,
    std_err_mean: f64,
    degrees_of_freedom: f64,
) -> (f64, f64) {
    let t = t_value(confidence, degrees_of_freedom);
    let span = t * std_err_mean;
    (mean - span, mean + span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

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
                assert_approx_eq!(t_value, z_value, 1e-2);
            }
        }
    }
}
