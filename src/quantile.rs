//! Confidence intervals for quantiles
//!
//! # Examples
//!
//! ```
//! # fn main() -> stats_ci::error::CIResult<()> {
//! use stats_ci::{quantile,Confidence,Interval};
//! let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
//! let confidence = Confidence::new_two_sided(0.95);
//! let quantile = 0.5; // median
//! let interval = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval, Interval::new(4, 12).unwrap());
//!
//! let confidence = Confidence::new_two_sided(0.8);
//! let interval = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval, Interval::new(6, 10).unwrap());
//!
//! let confidence = Confidence::new_two_sided(0.5);
//! let quantile = 0.2; // 20th percentile
//! let interval = quantile::ci(confidence, &data, quantile).unwrap();
//! assert_eq!(interval, Interval::new(2, 5).unwrap());
//! # Ok(())
//! # }
//! ```
//!
use super::*;
use crate::stats::z_value;

/// compute the confidence interval for a given quantile, assuming that the data is already sorted.
/// this is the function to call if the data is known to be sorted,
/// or if the order of elements is meant to be their position in the slice (e.g., order of arrival).
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `sorted` - the sorted sample
/// * `quantile` - the quantile to compute the confidence interval for (must be in (0, 1))
///
/// # Output
///
/// * `Interval` - the confidence interval for the quantile
/// * `None` - if the number of samples is too small to compute a confidence interval, or if the interval falls outside the range of the data.
///
/// # Errors
///
/// * `TooFewSamples` - if the number of samples is too small to compute a confidence interval
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
/// * `InvalidQuantile` - if the quantile is not in (0, 1)
///
/// # Examples
///
/// ```
/// # use stats_ci::*;
/// # fn main() -> error::CIResult<()> {
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// let confidence = Confidence::new_two_sided(0.95);
/// let quantile = 0.5; // median
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, Interval::new(4, 12).unwrap());
///
/// let confidence = Confidence::new_two_sided(0.8);
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, Interval::new(6, 10).unwrap());
///
/// let confidence = Confidence::new_two_sided(0.5);
/// let quantile = 0.2; // 20th percentile
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, Interval::new(2, 5).unwrap());
/// # Ok(())
/// # }
/// ```
///
pub fn ci_sorted_unchecked<T: PartialOrd + Clone>(
    confidence: Confidence,
    sorted: &[T],
    quantile: f64,
) -> Option<Interval<T>> {
    assert!(quantile > 0. && quantile < 1.);

    ci_indices(confidence, sorted.len(), quantile).and_then(|indices| match indices.into() {
        (Some(lo), Some(hi)) => Interval::new(sorted[lo].clone(), sorted[hi].clone()).ok(),
        (Some(lo), None) => Some(Interval::new_upper(sorted[lo].clone())),
        (None, Some(hi)) => Some(Interval::new_lower(sorted[hi].clone())),
        _ => None,
    })
}

/// compute the confidence interval for a given quantile
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `data` - the sample
/// * `quantile` - the quantile to compute the confidence interval for (must be in (0, 1))
///
/// # Errors
///
/// * `TooFewSamples` - if the number of samples is too small to compute a confidence interval
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
/// * `InvalidQuantile` - if the quantile is not in (0, 1)
///
/// # Panics
///
/// * if the data contains elements that are not comparable (with their partial ordering).
///
/// # Examples
///
/// ```
/// # fn main() -> stats_ci::CIResult<()> {
/// # use stats_ci::*;
/// let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
/// let confidence = Confidence::new_two_sided(0.95);
/// let quantile = 0.5; // median
/// let interval = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, Interval::new(4, 12)?);
///
/// let data2 = [2, 14, 13, 6, 8, 4, 15, 9, 3, 11, 10, 7, 1, 12, 5];
/// let interval2 = quantile::ci(confidence, &data2, quantile).unwrap();
/// assert_eq!(interval, interval2);
///
/// let confidence = Confidence::new_two_sided(0.8);
/// let interval = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, Interval::new(6, 10)?);
///
/// let confidence = Confidence::new_two_sided(0.5);
/// let quantile = 0.2; // 20th percentile
/// let interval = quantile::ci(confidence, &data, quantile).unwrap();
/// assert_eq!(interval, Interval::new(2, 5)?);
/// # Ok(())
/// # }
/// ```
pub fn ci<T: PartialOrd + Clone>(
    confidence: Confidence,
    data: &[T],
    quantile: f64,
) -> Option<Interval<T>> {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ci_sorted_unchecked(confidence, &sorted, quantile)
}

/// compute the confidence interval for a given quantile, assuming that the data is already sorted.
/// The function returns the indices of the lower and upper bounds of the interval.
/// this is the function to call if the data is known to be sorted,
/// or if the order of elements is meant to be their position in the slice (e.g., order of arrival).
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `sorted` - the sorted sample
/// * `quantile` - the quantile to compute the confidence interval for (must be in (0, 1))
///
/// # Output
///
/// * `Interval` - the confidence interval for the quantile
/// * `None` - if the number of samples is too small to compute a confidence interval, or if the interval falls outside the range of the data.
///
/// # Errors
///
/// * `TooFewSamples` - if the number of samples is too small to compute a confidence interval
/// * `InvalidConfidenceLevel` - if the confidence level is not in (0, 1)
/// * `InvalidQuantile` - if the quantile is not in (0, 1)
///
/// # Examples
///
/// ```
/// # use stats_ci::*;
/// # fn main() -> error::CIResult<()> {
/// let data = ["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O"];
/// let confidence = Confidence::new_two_sided(0.95);
/// let quantile = 0.5; // median
/// let interval = quantile::ci_indices(confidence, data.len(), quantile).unwrap();
/// assert_eq!(interval, Interval::new(3, 11).unwrap());
///
/// let confidence = Confidence::new_two_sided(0.8);
/// let interval = quantile::ci_indices(confidence, data.len(), quantile).unwrap();
/// assert_eq!(interval, Interval::new(5, 9).unwrap());
///
/// let confidence = Confidence::new_two_sided(0.5);
/// let quantile = 0.2; // 20th percentile
/// let interval = quantile::ci_indices(confidence, data.len(), quantile).unwrap();
/// assert_eq!(interval, Interval::new(1, 4).unwrap());
/// # Ok(())
/// # }
/// ```
///
pub fn ci_indices(
    confidence: Confidence,
    data_len: usize,
    quantile: f64,
) -> Option<Interval<usize>> {
    assert!(quantile > 0. && quantile < 1.);

    if data_len < 3 {
        // too few smaples to compute
        return None;
    }

    let z = z_value(confidence);
    let q = quantile; /* 0.5 for median */
    let n = data_len as f64;
    let mid_span = z * f64::sqrt(n * q * (1. - q));
    let lo_index = f64::ceil(n * q - mid_span) - 1.;
    let hi_index = f64::ceil(n * q + mid_span) - 1.;
    if lo_index < 0. {
        // interval falls outside the range of the data
        return None;
    }

    let lo_index = lo_index as usize;
    let hi_index = hi_index as usize;
    if hi_index >= data_len {
        // interval falls outside the range of the data
        return None;
    }

    match confidence {
        Confidence::TwoSided(_) => Interval::new(lo_index, hi_index).ok(),
        Confidence::UpperOneSided(_) => Some(Interval::new_upper(lo_index)),
        Confidence::LowerOneSided(_) => Some(Interval::new_lower(hi_index)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_median_ci() {
        let data = [
            8., 11., 12., 13., 15., 17., 19., 20., 21., 21., 22., 23., 25., 26., 28.,
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let median_ci = ci_sorted_unchecked(confidence, &data, 0.5);
        assert_eq!(median_ci, Interval::new(13., 23.).ok());
    }

    #[test]
    fn test_quantile_ci() {
        let data = [
            8., 11., 12., 13., 15., 17., 19., 20., 21., 21., 22., 23., 25., 26., 28.,
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile_ci = ci_sorted_unchecked(confidence, &data, 0.25);
        assert_eq!(quantile_ci, Interval::new(8., 20.).ok());

        let data = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci_indices(confidence, data.len(), quantile);
        assert_eq!(interval, Interval::new(3, 11).ok());

        let confidence = Confidence::new_two_sided(0.8);
        let interval = quantile::ci_indices(confidence, data.len(), quantile);
        assert_eq!(interval, Interval::new(5, 9).ok());

        let confidence = Confidence::new_two_sided(0.5);
        let quantile = 0.2; // 20th percentile
        let interval = quantile::ci_indices(confidence, data.len(), quantile);
        assert_eq!(interval, Interval::new(1, 4).ok());

        let data = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile);
        assert_eq!(interval, Interval::new("D", "L").ok());

        let data = [
            'J', 'E', 'M', 'G', 'K', 'H', 'N', 'A', 'C', 'L', 'F', 'O', 'D', 'B', 'I',
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci(confidence, &data, quantile);
        assert_eq!(interval, Interval::new('D', 'L').ok());
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Numbers {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Eleven,
        Twelve,
        Thirteen,
        Fourteen,
        Fifteen,
    }

    #[test]
    fn test_median_undordered() {
        use Numbers::*;
        let data = [
            One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Eleven, Twelve, Thirteen,
            Fourteen, Fifteen,
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let median_ci = ci_indices(confidence, data.len(), 0.5).unwrap();
        assert_eq!(median_ci, Interval::new(3, 11).unwrap());
        assert_eq!(median_ci.left(), Some(&3));
        assert_eq!(median_ci.right(), Some(&11));
    }

    #[test]
    fn test_median_ci_unsorted() {
        use rand::seq::SliceRandom;
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        for _i in 0..100 {
            let mut shuffled = data.to_vec();
            shuffled.shuffle(&mut thread_rng());
            let interval = ci(confidence, &shuffled, quantile);
            assert_eq!(interval, Interval::new(4, 12).ok());
        }
    }
}
