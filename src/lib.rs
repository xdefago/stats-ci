pub mod error;
mod interval;

pub mod mean;
pub mod proportion;
pub mod quantile;

pub use interval::Interval;

use lazy_static::lazy_static;
use statrs::distribution::ContinuousCDF;
use statrs::distribution::{Normal, StudentsT};

///
/// return the z-value of the normal distribution for a given confidence level.
///
/// # Arguments
///
/// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
/// * `two_sided` - if true, return the z-value for a two-sided test, otherwise return the z-value for a one-sided test
///
/// # Panics
///
/// * if `confidence` is not in the range (0, 1)
///
pub fn z_value(confidence: f64, two_sided: bool) -> f64 {
    lazy_static! {
        static ref NORMAL: Normal = Normal::new(0., 1.).unwrap();
    }
    assert!(confidence > 0. && confidence < 1.);
    let alpha = 1. - confidence;
    let alpha_prime = if two_sided { alpha / 2. } else { alpha };
    NORMAL.inverse_cdf(1. - alpha_prime)
}

fn z_value_two_sided(confidence: f64) -> f64 {
    z_value(confidence, true)
}

///
/// return the t-value of the t-distribution for a given confidence level.
///
/// # Arguments
///
/// * `confidence` - the confidence level, e.g. 0.95 for 95% confidence
/// * `degrees_of_freedom` - the degrees of freedom of the t-distribution
/// * `two_sided` - if true, return the t-value for a two-sided test, otherwise return the t-value for a one-sided test
///
/// # Panics
///
/// * if `confidence` is not in the range (0, 1)
///
fn t_value(confidence: f64, degrees_of_freedom: usize, two_sided: bool) -> f64 {
    assert!(confidence > 0. && confidence < 1.);
    let alpha = 1. - confidence;
    let student_t = StudentsT::new(0., 1., degrees_of_freedom as f64).unwrap();
    let alpha_prime = if two_sided { alpha / 2. } else { alpha };
    student_t.inverse_cdf(1. - alpha_prime)
}

fn t_value_two_sided(confidence: f64, degrees_of_freedom: usize) -> f64 {
    t_value(confidence, degrees_of_freedom, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_t_value() {
        for confidence in [0.5, 0.8, 0.9, 0.95, 0.99, 0.999] {
            for two_sided in [true, false] {
                let t_value = t_value(confidence, 1000, two_sided);
                let z_value = z_value(confidence, two_sided);
                assert_approx_eq!(t_value, z_value, 1e-2);
            }
        }
    }
}
