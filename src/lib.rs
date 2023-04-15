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
//! let ci = mean::Arithmetic::ci(confidence, data).unwrap();
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
//! use stats_ci::*;
//!
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
//! let confidence = Confidence::new_two_sided(0.95);
//! let quantile = 0.5; // median
//! let interval = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval, Interval::new(4, 12));
//!
//! let confidence = Confidence::new_two_sided(0.8);
//! let interval2 = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval2, Interval::new(6, 10));
//!
//! let data = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O"];
//! let confidence = Confidence::new_two_sided(0.95);
//! let quantile = 0.5; // median
//! let interval3 = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval3, Interval::new("D", "L"));
//! ```
//!
//! ## C.I. for Proportions
//!
//! ```
//! use stats_ci::*;
//! use assert_approx_eq::assert_approx_eq;
//!
//! let data = [
//!     true, false, true, true, false, true, true, false, true, true,
//!     false, false, false, true, false, true, false, false, true, false
//! ];
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci_true(confidence, data).unwrap();
//! assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
//!
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci_if(confidence, &data, |&x| x <= 10).unwrap();
//! assert_approx_eq!(interval.low().unwrap(), 0.299, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.701, 1e-2);
//!
//! let population = 500;
//! let successes = 421;
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci(confidence, population, successes).unwrap();
//! assert_approx_eq!(interval.low().unwrap(), 0.81, 1e-2);
//! assert_approx_eq!(interval.high().unwrap(), 0.87, 1e-2);
//! ```
//!
//! # References
//!

#![crate_type = "lib"]
#![crate_name = "stats_ci"]
#![forbid(unsafe_code)]

pub mod error;
pub mod mean;
pub mod proportion;
pub mod quantile;

mod confidence;
mod interval;

pub use confidence::Confidence;
pub use interval::Interval;
pub use mean::MeanCI;

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
/// * `two_sided` - if true, return the t-value for a two-sided test, otherwise return the t-value for a one-sided test
///
/// # Panics
///
/// * if `confidence` is not in the range (0, 1)
/// * if `degrees_of_freedom` is less than 1
///
pub fn t_value(confidence: Confidence, degrees_of_freedom: usize) -> f64 {
    let student_t = StudentsT::new(0., 1., degrees_of_freedom as f64).unwrap();
    student_t.inverse_cdf(confidence.quantile())
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_t_value() {
        for confidence_level in [0.5, 0.8, 0.9, 0.95, 0.99, 0.999] {
            for new_confidence in [
                Confidence::new_two_sided,
                Confidence::new_upper,
                Confidence::new_lower,
            ] {
                let confidence = new_confidence(confidence_level);
                let t_value = t_value(confidence, 1000);
                let z_value = z_value(confidence);
                assert_approx_eq!(t_value, z_value, 1e-2);
            }
        }
    }
}
