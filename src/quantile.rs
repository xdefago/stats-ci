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
//! let interval = quantile::ci(confidence, &data, quantile)?;
//! assert_eq!(interval, Interval::new(5, 12)?);
//!
//! let confidence = Confidence::new_two_sided(0.8);
//! let interval = quantile::ci(confidence, &data, quantile)?;
//! assert_eq!(interval, Interval::new(6, 11)?);
//!
//! let confidence = Confidence::new_two_sided(0.5);
//! let quantile = 0.4; // 40th percentile
//! let interval = quantile::ci(confidence, &data, quantile)?;
//! assert_eq!(interval, Interval::new(5, 8)?);
//! # Ok(())
//! # }
//! ```
//!
use super::*;

/// Compute the confidence interval for a given quantile, assuming that the data is __already sorted__.
/// This is the function to call if the data is known to be sorted,
/// or if the order of elements is meant to be their position in the slice (e.g., order of arrival).
///
/// Complexity: \\( O(1) \\)
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
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile)?;
/// assert_eq!(interval, Interval::new(5, 12)?);
///
/// let confidence = Confidence::new_two_sided(0.8);
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile)?;
/// assert_eq!(interval, Interval::new(6, 11)?);
///
/// let confidence = Confidence::new_two_sided(0.5);
/// let quantile = 0.4; // 40th percentile
/// let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile)?;
/// assert_eq!(interval, Interval::new(5, 8)?);
/// # Ok(())
/// # }
/// ```
///
pub fn ci_sorted_unchecked<T: PartialOrd + Clone>(
    confidence: Confidence,
    sorted: &[T],
    quantile: f64,
) -> CIResult<Interval<T>> {
    assert!(quantile > 0. && quantile < 1.);

    ci_indices(confidence, sorted.len(), quantile).and_then(|indices| match indices.into() {
        (Some(lo), Some(hi)) => {
            Interval::new(sorted[lo].clone(), sorted[hi].clone()).map_err(|e| e.into())
        }
        (Some(lo), None) => Ok(Interval::new_upper(sorted[lo].clone())),
        (None, Some(hi)) => Ok(Interval::new_lower(sorted[hi].clone())),
        _ => Err(error::CIError::IntervalError(
            interval::IntervalError::EmptyInterval,
        )),
    })
}

/// Compute the confidence interval for a given quantile.
/// Use [`ci_sorted_unchecked`] instead if the data is already sorted.
///
/// Complexity: \\( O(n \log n) \\) where \\( n \\) is the number of samples.
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
/// let interval = quantile::ci(confidence, &data, quantile)?;
/// assert_eq!(interval, Interval::new(5, 12)?);
///
/// let data2 = [2, 14, 13, 6, 8, 4, 15, 9, 3, 11, 10, 7, 1, 12, 5];
/// let interval2 = quantile::ci(confidence, &data2, quantile)?;
/// assert_eq!(interval, interval2);
///
/// let confidence = Confidence::new_two_sided(0.8);
/// let interval = quantile::ci(confidence, &data, quantile)?;
/// assert_eq!(interval, Interval::new(6, 11)?);
///
/// let confidence = Confidence::new_two_sided(0.5);
/// let quantile = 0.4; // 40th percentile
/// let interval = quantile::ci(confidence, &data, quantile)?;
/// assert_eq!(interval, Interval::new(5, 8)?);
/// # Ok(())
/// # }
/// ```
pub fn ci<T: PartialOrd + Clone>(
    confidence: Confidence,
    data: &[T],
    quantile: f64,
) -> CIResult<Interval<T>> {
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    ci_sorted_unchecked(confidence, &sorted, quantile)
}

/// Compute the indices of the confidence interval for a given quantile.
/// The function returns the indices of the lower and upper bounds of the interval.
///
/// Complexity: \\( O(1) \\)
///
/// # Arguments
///
/// * `confidence` - the confidence level (must be in (0, 1))
/// * `data_len` - the number of samples
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
/// let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
/// assert_eq!(interval, Interval::new(4, 11)?);
///
/// let confidence = Confidence::new_two_sided(0.8);
/// let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
/// assert_eq!(interval, Interval::new(5, 10)?);
///
/// let confidence = Confidence::new_two_sided(0.5);
/// let quantile = 0.4; // 40th percentile
/// let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
/// assert_eq!(interval, Interval::new(4, 7)?);
/// # Ok(())
/// # }
/// ```
///
pub fn ci_indices(
    confidence: Confidence,
    data_len: usize,
    quantile: f64,
) -> CIResult<Interval<usize>> {
    assert!(quantile > 0. && quantile < 1.);

    if data_len < 3 {
        // too few samples to compute
        return Err(error::CIError::TooFewSamples(data_len));
    }

    let successes = (quantile * data_len as f64).round() as usize;
    let proportion_ci = proportion::ci_wilson(confidence, data_len, successes)?;

    let (low, high): (f64, f64) = proportion_ci.into();

    if low < 0. {
        // interval falls outside the range of the data
        return Err(error::CIError::IndexError(low, data_len));
    }

    let lo_index = (low * data_len as f64).floor() as usize;
    let hi_index = (high * data_len as f64).floor() as usize;

    if hi_index > data_len {
        // interval falls outside the range of the data
        return Err(error::CIError::IndexError(hi_index as f64, data_len));
    }

    let hi_index = hi_index.min(data_len - 1);

    match confidence {
        Confidence::TwoSided(_) => Interval::new(lo_index, hi_index).map_err(|e| e.into()),
        Confidence::UpperOneSided(_) => Ok(Interval::new_upper(lo_index)),
        Confidence::LowerOneSided(_) => Ok(Interval::new_lower(hi_index)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_median_ci() -> CIResult<()> {
        let data = [
            8., 11., 12., 13., 15., 17., 19., 20., 21., 21., 22., 23., 25., 26., 28.,
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let median_ci = ci_sorted_unchecked(confidence, &data, 0.5)?;
        assert_eq!(median_ci, Interval::new(15., 23.)?);

        let confidence = Confidence::new_lower(0.975);
        let median_ci = ci_sorted_unchecked(confidence, &data, 0.5)?;
        assert_eq!(median_ci, Interval::new_lower(23.));

        let confidence = Confidence::new_upper(0.975);
        let median_ci = ci_sorted_unchecked(confidence, &data, 0.5)?;
        assert_eq!(median_ci, Interval::new_upper(15.));

        Ok(())
    }

    #[test]
    fn test_quantile_ci() -> CIResult<()> {
        let data = [
            8., 11., 12., 13., 15., 17., 19., 20., 21., 21., 22., 23., 25., 26., 28.,
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile_ci = ci_sorted_unchecked(confidence, &data, 0.4).unwrap();
        assert_eq!(quantile_ci, Interval::new(12., 21.)?);

        let data = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci_indices(confidence, data.len(), quantile).unwrap();
        assert_eq!(interval, Interval::new(4, 11)?);

        let confidence = Confidence::new_two_sided(0.8);
        let interval = quantile::ci_indices(confidence, data.len(), quantile).unwrap();
        assert_eq!(interval, Interval::new(5, 10)?);

        let confidence = Confidence::new_two_sided(0.5);
        let quantile = 0.4; // 40th percentile
        let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
        assert_eq!(interval, Interval::new(4, 7)?);

        let data = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci_sorted_unchecked(confidence, &data, quantile)?;
        assert_eq!(interval, Interval::new("E", "L")?);

        let data = [
            'J', 'E', 'M', 'G', 'K', 'H', 'N', 'A', 'C', 'L', 'F', 'O', 'D', 'B', 'I',
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci(confidence, &data, quantile)?;
        assert_eq!(interval, Interval::new('E', 'L')?);

        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci(confidence, &data, quantile)?;
        assert_eq!(interval, Interval::new(5, 12)?);

        let confidence = Confidence::new_two_sided(0.8);
        let interval = quantile::ci(confidence, &data, quantile)?;
        assert_eq!(interval, Interval::new(6, 11)?);

        let confidence = Confidence::new_two_sided(0.5);
        let quantile = 0.4; // 40th percentile
        let interval = quantile::ci(confidence, &data, quantile)?;
        assert_eq!(interval, Interval::new(5, 8)?);

        Ok(())
    }

    #[test]
    fn test_ci_indices() -> CIResult<()> {
        let data = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
        assert_eq!(interval, Interval::new(4, 11)?);

        let confidence = Confidence::new_two_sided(0.8);
        let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
        assert_eq!(interval, Interval::new(5, 10)?);

        let confidence = Confidence::new_two_sided(0.5);
        let quantile = 0.4; // 40th percentile
        let interval = quantile::ci_indices(confidence, data.len(), quantile)?;
        assert_eq!(interval, Interval::new(4, 7)?);

        Ok(())
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
    fn test_median_undordered() -> CIResult<()> {
        use Numbers::*;
        let data = [
            One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Eleven, Twelve, Thirteen,
            Fourteen, Fifteen,
        ];
        let confidence = Confidence::new_two_sided(0.95);
        let median_ci = ci_indices(confidence, data.len(), 0.5)?;
        assert_eq!(median_ci, Interval::new(4, 11)?);
        assert_eq!(median_ci.left(), Some(&4));
        assert_eq!(median_ci.right(), Some(&11));

        Ok(())
    }

    #[test]
    fn test_median_ci_unsorted() -> CIResult<()> {
        use rand::seq::SliceRandom;
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let confidence = Confidence::new_two_sided(0.95);
        let quantile = 0.5; // median
        for _i in 0..100 {
            let mut shuffled = data.to_vec();
            shuffled.shuffle(&mut thread_rng());
            let interval = ci(confidence, &shuffled, quantile)?;
            assert_eq!(interval, Interval::new(5, 12)?);
        }
        Ok(())
    }
}
