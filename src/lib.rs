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
//!
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
//! # use assert_approx_eq::assert_approx_eq;
//! use stats_ci::*;
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
//! * Raj Jain. [The Art of Computer Systems Performance Analysis: Techniques for Experimental Design, Measurement, Simulation, and Modeling,](https://www.cse.wustl.edu/~jain/books/perfbook.htm) John Wiley & Sons, 1991.
//! * [Wikipedia - Confidence interval](https://en.wikipedia.org/wiki/Confidence_interval)
//! * [Wikipedia - Binomial proportion confidence interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval)
//! * [Wikipedia article on normal approximation interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval)
//! * Francis J. DiTraglia. [Blog post: The Normal Approximation Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-normal-approximation-confidence-interval-for-a-proportion/)

#![crate_type = "lib"]
#![crate_name = "stats_ci"]
#![forbid(unsafe_code)]

pub mod error;
pub mod mean;
pub mod proportion;
pub mod quantile;

mod confidence;
mod interval;
mod stats;

pub use confidence::Confidence;
pub use interval::Interval;
pub use mean::MeanCI;
