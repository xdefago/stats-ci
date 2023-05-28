#![doc = include_str!("../README.md")]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_readme() {
        // 2. collect the data
        let data = [
            10.6, 6.6, 26.7, 0.4, 5.7, 0.3, 1.1, 5.0, 8.4, 1.4, 15.1, 0.3, 20.4, 1.2, 28.4, 10.7,
            0.4, 10.1, 4.5, 7.1, 4.3, 37.4, 0.9, 10.1, 12.6, 21.7, 21.9, 2.0, 8.4, 9.3,
        ];
        // 3. define the confidence level (for 95% confidence)
        let confidence = Confidence::new(0.95);

        // 4a. compute the interval for the arithmetic mean
        if let Ok(ci) = mean::Arithmetic::<f64>::ci(confidence, data) {
            // display the interval
            println!("{}% c.i. for the mean = {}", confidence.percent(), ci);
            if !ci.contains(&10.) {
                println!("Does NOT contains the theoretical mean!");
            }
        }
        // 4b. compute the interval for the median (i.e., 0.5-quantile)
        if let Ok(ci) = quantile::ci(confidence, data, 0.5) {
            // display the interval
            println!("{}% c.i. for the median = {}", confidence.percent(), ci);
            if !ci.contains(&6.93147) {
                println!("Does NOT contains the theoretical median!");
            }
        }
    }
}
