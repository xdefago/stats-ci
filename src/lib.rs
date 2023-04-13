//! This crate aims to provide convenient functions to compute confidence intervals
//! in various situations. The motivation comes from a personal need and was that no
//! crate seem to provide an easy and comprehensive solution to computing such intervals.
//! One exception is the crate `criterion` which computes confidence intervals for its
//! measurements but does not export such functionality.
//!
//! This crate exports a type [`Interval`] to express the confidence intervals. Intervals
//! are generic and can be instantiated for various types, beyond the usual float or integer
//! types.
//!
//! The crate provides functions to compute various kinds of confidence intervals:
//! * intervals over the mean of floating-point data
//! * intervals over quantiles (incl. median) of sample data of any ordered types
//! * intervals on proportions on Boolean data (arrays or iterators).
//!
//! # Examples
//!
//! ## C.I. for the Mean
//!
//! ```
//! use stats_ci::mean;
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
//!
//! ## C.I. for Quantiles
//!
//! ```
//! use stats_ci::quantile;
//! use stats_ci::Interval;
//!
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
//! let confidence = 0.95;
//! let quantile = 0.5; // median
//! let interval = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval, Interval::new(4, 12));
//!
//! let confidence = 0.8;
//! let interval2 = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval2, Interval::new(6, 10));
//!
//! let data = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O"];
//! let confidence = 0.95;
//! let quantile = 0.5; // median
//! let interval3 = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval3, Interval::new("D", "L"));
//! ```
//!
//! ## C.I. for Proportions
//!
//! ```
//! use stats_ci::proportion;
//! use assert_approx_eq::assert_approx_eq;
//!
//! let data = [
//!     true, false, true, true, false, true, true, false, true, true,
//!     false, false, false, true, false, true, false, false, true, false
//! ];
//! let confidence = 0.95;
//! let interval = proportion::ci_true(confidence, data).unwrap();
//! assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
//!
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
//! let confidence = 0.95;
//! let interval = proportion::ci_if(confidence, &data, |&x| x <= 10).unwrap();
//! assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
//!
//! let population = 500;
//! let successes = 421;
//! let confidence = 0.95;
//! let interval = proportion::ci(confidence, population, successes).unwrap();
//! assert_approx_eq!(interval.low().unwrap(), 0.81, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.87, 1e-2);
//! ```
//!
//! # References
//!

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
