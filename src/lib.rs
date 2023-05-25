//! This crate aims to provide convenient functions to compute confidence intervals
//! in various situations. The motivation comes from a personal need and was that no
//! crate seem to provide an easy and comprehensive solution to computing such intervals.
//! One exception is the crate `criterion` which computes confidence intervals for its
//! measurements but does not export such functionality.
//!
//! This crate exports a type [`Confidence`] to express a confidence level and
//! a type [`Interval`] to represent a confidence interval.
//! Intervals are generic and can be instantiated for various types, beyond the usual
//! float or integer types.
//!
//! The crate provides functions to compute various kinds of confidence intervals:
//! * intervals over the mean of floating-point data
//! * intervals over quantiles (incl. median) of sample data of any ordered types
//! * intervals on proportions on Boolean data (arrays or iterators).
//! * comparison of two means (paired or unpaired observations).
//!
//! # Examples
//!
//! ## C.I. for the Mean
//!
//! The crate provides functions to compute confidence intervals for the mean of floating-point (`f32` or `f64`) data.
//! The functions are generic and can be used with any type implementing the [`num_traits::Float`] trait from the crate [`num_traits`].
//! When dealing with integer data it is necessary to convert it to floating-point values.
//!
//! The crate provides three functions to compute confidence intervals for the mean of floating-point data:
//! * [`mean::Arithmetic`] computes the confidence interval for the arithmetic mean.
//! * [`mean::Geometric`] computes the confidence interval for the geometric mean
//! * [`mean::Harmonic`] computes the confidence interval for the harmonic mean
//!
//! The functionality is mainly provided by the trait [`StatisticsOps`] (preferred) or the trait [`MeanCI`] (legacy) on the above structs.
//!
//! ```
//! # fn main() -> stats_ci::CIResult<()> {
//! use stats_ci::*;
//! use approx::*;
//!
//! let data = [
//!     82., 94., 68., 6., 39., 80., 10., 97., 34., 66., 62., 7., 39., 68., 93., 64., 10., 74.,
//!     15., 34., 4., 48., 88., 94., 17., 99., 81., 37., 68., 66., 40., 23., 67., 72., 63.,
//!     71., 18., 51., 65., 87., 12., 44., 89., 67., 28., 86., 62., 22., 90., 18., 50., 25.,
//!     98., 24., 61., 62., 86., 100., 96., 27., 36., 82., 90., 55., 26., 38., 97., 73., 16.,
//!     49., 23., 26., 55., 26., 3., 23., 47., 27., 58., 27., 97., 32., 29., 56., 28., 23.,
//!     37., 72., 62., 77., 63., 100., 40., 84., 77., 39., 71., 61., 17., 77.,
//! ];
//!
//! // 1. create a statistics object
//! let mut stats = mean::Arithmetic::new();
//! // 2. add data
//! stats.extend(data)?;
//! // 3. compute the confidence intervals over the mean for some confidence level
//! let ci_95 = stats.ci_mean(Confidence::new_two_sided(0.95))?;
//! let ci_lower = stats.ci_mean(Confidence::new_lower(0.975))?;
//! // 4. get other statistics on the sample data
//! assert_abs_diff_eq!(stats.sample_mean(), 53.67);
//! assert_abs_diff_eq!(stats.sample_std_dev(), 28.097613040716794);
//! // reference values computed in python/numpy
//! // [48.094823990767836, 59.24517600923217]
//! assert_abs_diff_eq!(ci_95.low_f(), 48.094823990767836, epsilon = 1e-6);
//! assert_abs_diff_eq!(ci_95.high_f(), 59.24517600923217, epsilon = 1e-6);
//!
//! assert_abs_diff_eq!(ci_95, Interval::new(48.09482399055084, 59.24517600944916)?);
//! assert_abs_diff_eq!(ci_lower, Interval::new_lower(59.24517600944916));
//! # Ok(())
//! # }
//! ```
//!
//! It is also possible to compute the confidence intervals incrementally by using
//! the [`mean::Arithmetic::new`] constructor or the [`mean::Arithmetic::from_iter`].
//!
//! ## C.I. for Quantiles
//!
//! Depending on the type of data and measurements, it is sometimes inappropriate to compute the mean of the data because that value makes little sense.
//! For instance, consider a communication system and suppose that we want to test if at least 95% of messages are delivered within 1 second with 90% confidence.
//! Then, the value of interest is the one-sided confidence interval of the 95th percentile (quantile=.95, confidence level=0.9).
//!
//! In a different context, if the data is an ordered sequence of strings, it could (in some context) make sense to compute an interval around the median of the data, but the mean cannot be computed.
//!
//! ```
//! use stats_ci::*;
//!
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
//! let confidence = Confidence::new_two_sided(0.95);
//! let quantile = 0.5; // median
//! let interval = quantile::ci(confidence, &data, quantile)?;
//! assert_eq!(interval, Interval::new(5, 12)?);
//!
//! let confidence = Confidence::new_two_sided(0.8);
//! let interval2 = quantile::ci(confidence, &data, quantile)?;
//! assert_eq!(interval2, Interval::new(6, 11)?);
//!
//! let data = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O"];
//! let confidence = Confidence::new_two_sided(0.95);
//! let quantile = 0.5; // median
//! let interval3 = quantile::ci(confidence, &data, quantile)?;
//! assert_eq!(interval3, Interval::new("E", "L")?);
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! ## C.I. for Proportions
//!
//! Confidence intervals for proportions are often used in the context of A/B testing or when measuring the success/failure rate of a system.
//! It is also useful when running Monte-Carlo simulations to estimate the winning chances of a player in a game.
//!
//! This crate uses the Wilson score interval to compute the confidence interval for a proportion,
//! which is more stable than the standard normal approximation but results in slightly more conservative intervals.
//! ```
//! use stats_ci::*;
//! use approx::*;
//!
//! let data = [
//!     true, false, true, true, false, true, true, false, true, true,
//!     false, false, false, true, false, true, false, false, true, false
//! ];
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci_true(confidence, data)?;
//! assert_abs_diff_eq!(interval.low().unwrap(), 0.299, epsilon = 1e-2);
//! assert_abs_diff_eq!(interval.high().unwrap(), 0.701, epsilon = 1e-2);
//!
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci_if(confidence, &data, |&x| x <= 10)?;
//! assert_abs_diff_eq!(interval.low().unwrap(), 0.299, epsilon = 1e-2);
//! assert_abs_diff_eq!(interval.high().unwrap(), 0.701, epsilon = 1e-2);
//!
//! let population = 500;
//! let successes = 421;
//! let confidence = Confidence::new_two_sided(0.95);
//! let interval = proportion::ci(confidence, population, successes)?;
//! assert_abs_diff_eq!(interval.low().unwrap(), 0.81, epsilon = 1e-2);
//! assert_abs_diff_eq!(interval.high().unwrap(), 0.87, epsilon = 1e-2);
//! # Ok::<(),error::CIError>(())
//! ```
//!
//! # References
//!
//! * Raj Jain. [The Art of Computer Systems Performance Analysis: Techniques for Experimental Design, Measurement, Simulation, and Modeling,](https://www.cse.wustl.edu/~jain/books/perfbook.htm) John Wiley & Sons, 1991.
//! * [Wikipedia - Confidence interval](https://en.wikipedia.org/wiki/Confidence_interval)
//! * [Wikipedia - Binomial proportion confidence interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval)
//! * [Wikipedia article on normal approximation interval](https://en.wikipedia.org/wiki/Binomial_proportion_confidence_interval#Normal_approximation_interval)
//! * Dransfield R.D., Brightwell R. (2012) Avoiding and Detecting Statistical Malpractice (or "How to Get On Top of Statistics): Design & Analysis for Biologists, with R. InfluentialPoints, UK [online](https://influentialpoints.com/hyperbook.htm)
//! * _idem_. Chapter [Confidence intervals of proportions and rates](https://influentialpoints.com/Training/confidence_intervals_of_proportions-principles-properties-assumptions.htm)
//! * Francis J. DiTraglia. [Blog post: The Wilson Confidence Interval for a Proportion](https://www.econometrics.blog/post/the-wilson-confidence-interval-for-a-proportion/). Feb 2022.
//! * Nilan Noris. "The standard errors of the geometric and harmonic means and their application to index numbers." Ann. Math. Statist. 11(4): 445-448 (December, 1940). DOI: [10.1214/aoms/1177731830](https://doi.org/10.1214/aoms/1177731830) [JSTOR](https://www.jstor.org/stable/2235727)
//! * PennState. Stat 500. [Online](https://online.stat.psu.edu/stat500/)
//!

#![crate_type = "lib"]
#![crate_name = "stats_ci"]
#![forbid(unsafe_code)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]
#![warn(missing_docs)]

pub mod comparison;
pub mod error;
pub mod mean;
pub mod proportion;
pub mod quantile;

pub mod utils;

mod confidence;
mod interval;
mod stats;

pub use confidence::Confidence;
pub use error::CIResult;
pub use interval::Interval;
pub use mean::MeanCI;
pub use mean::StatisticsOps;
